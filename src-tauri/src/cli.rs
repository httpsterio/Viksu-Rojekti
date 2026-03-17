use clap::{Parser, Subcommand};
use std::path::PathBuf;
use crate::models::{BoardConfig, Ticket, TicketMeta};
use crate::{storage, index};
use chrono::Local;
use std::io::Write;

#[derive(Parser)]
#[command(name = "rojekti")]
#[command(about = "Local kanban board CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Rebuild index.yaml from ticket files
    RebuildIndex,
    /// Renormalize position values in a lane
    Reorder {
        /// Lane to reorder
        #[arg(short, long)]
        lane: String,
    },
    /// List all tickets to stdout
    List,
    /// Print a single ticket to stdout
    Show {
        /// Ticket ID
        #[arg(short, long)]
        id: String,
    },
    /// Create a new ticket
    Create {
        /// Ticket title
        #[arg(short, long)]
        title: String,
        /// Lane (defaults to first lane)
        #[arg(short, long)]
        status: Option<String>,
        /// Priority level (defaults to medium)
        #[arg(short, long)]
        priority: Option<String>,
        /// Epic ID
        #[arg(short, long)]
        epic: Option<String>,
        /// Comma-separated tags
        #[arg(short, long)]
        tags: Option<String>,
    },
    /// Initialize a new board in the current directory
    Init {
        /// Board name
        #[arg(short, long)]
        name: String,
        /// Ticket ID prefix
        #[arg(short, long)]
        prefix: String,
    },
}

pub fn handle_cli(command: Commands, project_dir: PathBuf) {
    match command {
        Commands::RebuildIndex => {
            match index::rebuild_index(&project_dir) {
                Ok(_) => println!("Index rebuilt successfully."),
                Err(e) => eprintln!("Error rebuilding index: {}", e),
            }
        }
        Commands::Reorder { lane } => {
            if lane.is_empty() {
                eprintln!("Error: lane is required.");
            } else {
                match storage::read_all_tickets(&project_dir) {
                    Ok(tickets) => {
                        let mut lane_tickets: Vec<Ticket> = tickets.into_iter()
                            .filter(|t| t.meta.status == lane)
                            .collect();
                        lane_tickets.sort_by(|a, b| a.meta.position.partial_cmp(&b.meta.position).unwrap());
                        
                        for (i, mut ticket) in lane_tickets.into_iter().enumerate() {
                            ticket.meta.position = (i + 1) as f64;
                            if let Err(e) = storage::write_ticket(&project_dir, &ticket) {
                                eprintln!("Error updating ticket {}: {}", ticket.meta.id, e);
                            }
                        }
                        let _ = index::rebuild_index(&project_dir);
                        println!("Lane '{}' reordered.", lane);
                    }
                    Err(e) => eprintln!("Error reading tickets: {}", e),
                }
            }
        }
        Commands::List => {
            match storage::read_all_tickets(&project_dir) {
                Ok(mut tickets) => {
                    tickets.sort_by(|a, b| {
                        a.meta.status.cmp(&b.meta.status)
                            .then(a.meta.position.partial_cmp(&b.meta.position).unwrap())
                    });
                    for t in tickets {
                        println!("{}: {} [{}] ({})", t.meta.id, t.meta.title, t.meta.status, t.meta.priority);
                    }
                }
                Err(e) => eprintln!("Error listing tickets: {}", e),
            }
        }
        Commands::Show { id } => {
            if id.is_empty() {
                eprintln!("Error: id is required.");
            } else {
                match storage::read_ticket(&project_dir.join("tickets").join(format!("{}.md", id))) {
                    Ok(t) => {
                        println!("ID: {}", t.meta.id);
                        println!("Title: {}", t.meta.title);
                        println!("Status: {}", t.meta.status);
                        println!("Priority: {}", t.meta.priority);
                        if let Some(epic) = t.meta.epic { println!("Epic: {}", epic); }
                        println!("Tags: {}", t.meta.tags.join(", "));
                        println!("Created: {}", t.meta.created);
                        println!("\n--- Description ---\n");
                        println!("{}", t.body);
                    }
                    Err(e) => eprintln!("Error showing ticket {}: {}", id, e),
                }
            }
        }
        Commands::Create { title, status, priority, epic, tags } => {
            if title.is_empty() {
                eprintln!("Error: title is required.");
            } else {
                let priority_val = priority.unwrap_or_else(|| "medium".to_string());
                let tags_str = tags.unwrap_or_default();
                let tags_vec: Vec<String> = if tags_str.is_empty() { 
                    Vec::new() 
                } else { 
                    tags_str.split(',').map(|s| s.trim().to_string()).collect() 
                };

                match storage::read_board_config(&project_dir.join("board.yaml")) {
                    Ok(mut config) => {
                        let id = format!("{}-{:03}", config.prefix, config.next_id);
                        config.next_id += 1;
                        let _ = storage::write_board_config(&project_dir.join("board.yaml"), &config);
                        
                        let status_val = status.unwrap_or_else(|| config.lanes.first().cloned().unwrap_or_else(|| "todo".to_string()));
                        
                        let tickets = storage::read_all_tickets(&project_dir).unwrap_or_default();
                        let max_pos = tickets.iter()
                            .filter(|t| t.meta.status == status_val)
                            .map(|t| t.meta.position)
                            .fold(0.0, f64::max);

                        let ticket = Ticket {
                            meta: TicketMeta {
                                id: id.clone(),
                                title: title.to_string(),
                                status: status_val,
                                epic,
                                tags: tags_vec,
                                priority: priority_val,
                                position: max_pos + 1.0,
                                created: Local::now().format("%Y-%m-%d").to_string(),
                            },
                            body: "".to_string(),
                        };
                        
                        match storage::write_ticket(&project_dir, &ticket) {
                            Ok(_) => {
                                let _ = index::rebuild_index(&project_dir);
                                println!("Created ticket {}.", id);
                            }
                            Err(e) => eprintln!("Error creating ticket: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Error reading board.yaml: {}", e),
                }
            }
        }
        Commands::Init { name, prefix } => {
            if name.is_empty() || prefix.is_empty() {
                eprintln!("Error: name and prefix are required.");
            } else {
                let config = BoardConfig {
                    name: name.to_string(),
                    prefix: prefix.to_string(),
                    next_id: 1,
                    lanes: vec!["backlog".into(), "todo".into(), "in-progress".into(), "review".into(), "done".into()],
                    epics: Vec::new(),
                    tags: Vec::new(),
                    priorities: vec!["low".into(), "medium".into(), "high".into(), "critical".into()],
                };
                
                if !project_dir.exists() {
                    let _ = std::fs::create_dir_all(&project_dir);
                }
                
                if let Err(e) = storage::write_board_config(&project_dir.join("board.yaml"), &config) {
                    eprintln!("Error writing board.yaml: {}", e);
                } else {
                    let _ = std::fs::create_dir_all(project_dir.join("tickets"));
                    let _ = index::rebuild_index(&project_dir);
                    println!("Initialized board '{}' with prefix '{}'.", name, prefix);
                }
            }
        }
    }
    let _ = std::io::stdout().flush();
    let _ = std::io::stderr().flush();
}
