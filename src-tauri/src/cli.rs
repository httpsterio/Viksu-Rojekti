use clap::{Parser, Subcommand};
use std::path::PathBuf;
use crate::models::{BoardConfig, Card, CardMeta, Status, Priority};
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
    /// Rebuild rojekti.index.yaml from card files
    RebuildIndex,
    /// Renormalize position values in a status
    Reorder {
        /// Status to reorder
        #[arg(short, long)]
        status: String,
    },
    /// List all cards to stdout
    List,
    /// Print a single card to stdout
    Show {
        /// Card ID
        #[arg(short, long)]
        id: String,
    },
    /// Create a new card
    Create {
        /// Card title
        #[arg(short, long)]
        title: String,
        /// Status (defaults to first status)
        #[arg(short, long)]
        status: Option<String>,
        /// Priority level (1-5, defaults to 0/None)
        #[arg(short, long)]
        priority: Option<u8>,
        /// Epic name
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
        /// Card ID prefix
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
        Commands::Reorder { status } => {
            if status.is_empty() {
                eprintln!("Error: status is required.");
            } else {
                match storage::read_all_cards(&project_dir) {
                    Ok((cards, errors)) => {
                        if !errors.is_empty() {
                            for err in errors { eprintln!("Warning: {}", err); }
                        }
                        let mut status_cards: Vec<Card> = cards.into_iter()
                            .filter(|c| c.meta.status == *status)
                            .collect();
                        status_cards.sort_by(|a, b| a.meta.position.partial_cmp(&b.meta.position).unwrap());
                        
                        for (i, mut card) in status_cards.into_iter().enumerate() {
                            card.meta.position = (i + 1) as f64;
                            if let Err(e) = storage::write_card(&project_dir, &card) {
                                eprintln!("Error updating card {}: {}", card.meta.id, e);
                            }
                        }
                        let _ = index::rebuild_index(&project_dir);
                        println!("Status '{}' reordered.", status);
                    }
                    Err(e) => eprintln!("Error reading cards: {}", e),
                }
            }
        }
        Commands::List => {
            match storage::read_all_cards(&project_dir) {
                Ok((mut cards, errors)) => {
                    if !errors.is_empty() {
                        for err in errors { eprintln!("Warning: {}", err); }
                    }
                    cards.sort_by(|a, b| {
                        a.meta.status.cmp(&b.meta.status)
                            .then(a.meta.position.partial_cmp(&b.meta.position).unwrap())
                    });
                    for c in cards {
                        println!("{}: {} [{}] ({})", c.meta.id, c.meta.title, c.meta.status, c.meta.priority);
                    }
                }
                Err(e) => eprintln!("Error listing cards: {}", e),
            }
        }
        Commands::Show { id } => {
            if id.is_empty() {
                eprintln!("Error: id is required.");
            } else {
                match storage::read_card(&project_dir.join("rojekti").join("cards").join(format!("{}.md", id))) {
                    Ok(c) => {
                        println!("ID: {}", c.meta.id);
                        println!("Title: {}", c.meta.title);
                        println!("Status: {}", c.meta.status);
                        println!("Priority: {}", c.meta.priority);
                        if let Some(epic) = c.meta.epic { println!("Epic: {}", epic); }
                        println!("Tags: {}", c.meta.tags.join(", "));
                        println!("Created: {}", c.meta.created);
                        println!("\n--- Description ---\n");
                        println!("{}", c.body);
                    }
                    Err(e) => eprintln!("Error showing card {}: {}", id, e),
                }
            }
        }
        Commands::Create { title, status, priority, epic, tags } => {
            if title.is_empty() {
                eprintln!("Error: title is required.");
            } else {
                let priority_val = priority.unwrap_or(0);
                let tags_str = tags.unwrap_or_default();
                let tags_vec: Vec<String> = if tags_str.is_empty() { 
                    Vec::new() 
                } else { 
                    tags_str.split(',').map(|s| s.trim().to_string()).collect() 
                };

                match storage::read_board_config(&project_dir.join("rojekti").join("rojekti.config.yaml")) {
                    Ok(mut config) => {
                        let id = format!("{}-{:03}", config.prefix, config.next_id);
                        config.next_id += 1;
                        let _ = storage::write_board_config(&project_dir.join("rojekti").join("rojekti.config.yaml"), &config);
                        
                        let status_val = status.unwrap_or_else(|| {
                            config.statuses.first().map(|s| s.id.clone()).unwrap_or_else(|| "todo".to_string())
                        });
                        
                        let (cards, _) = storage::read_all_cards(&project_dir).unwrap_or_default();
                        let max_pos = cards.iter()
                            .filter(|c| c.meta.status == status_val)
                            .map(|c| c.meta.position)
                            .fold(0.0, f64::max);

                        let card = Card {
                            meta: CardMeta {
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
                        
                        match storage::write_card(&project_dir, &card) {
                            Ok(_) => {
                                let _ = index::rebuild_index(&project_dir);
                                println!("Created card {}.", id);
                            }
                            Err(e) => eprintln!("Error creating card: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Error reading config: {}", e),
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
                    statuses: vec![
                        Status { id: "backlog".into(), name: "Backlog".into() },
                        Status { id: "todo".into(), name: "Todo".into() },
                        Status { id: "in-progress".into(), name: "In Progress".into() },
                        Status { id: "review".into(), name: "Review".into() },
                        Status { id: "done".into(), name: "Done".into() },
                    ],
                    epics: Vec::new(),
                    tags: Vec::new(),
                    priorities: vec![
                        Priority { name: "Critical".into(),    color: "#e53e3e".into() },
                        Priority { name: "Severe".into(),      color: "#ed8936".into() },
                        Priority { name: "Substantial".into(), color: "#ecc94b".into() },
                        Priority { name: "Moderate".into(),    color: "#68d391".into() },
                        Priority { name: "Low".into(),         color: "#a0aec0".into() },
                    ],
                };
                
                let rojekti_dir = project_dir.join("rojekti");
                if !rojekti_dir.exists() {
                    let _ = std::fs::create_dir_all(&rojekti_dir);
                }
                
                if let Err(e) = storage::write_board_config(&rojekti_dir.join("rojekti.config.yaml"), &config) {
                    eprintln!("Error writing config: {}", e);
                } else {
                    let _ = std::fs::create_dir_all(rojekti_dir.join("cards"));
                    let _ = index::rebuild_index(&project_dir);
                    println!("Initialized board '{}' with prefix '{}'.", name, prefix);
                }
            }
        }
    }
    let _ = std::io::stdout().flush();
    let _ = std::io::stderr().flush();
}
