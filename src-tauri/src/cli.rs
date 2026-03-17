use tauri_plugin_cli::Matches;
use std::path::PathBuf;
use crate::models::{BoardConfig, Ticket, TicketMeta};
use crate::{storage, index};
use chrono::Local;

pub fn handle_cli(matches: Matches, project_dir: PathBuf) {
    if let Some(subcommand) = matches.subcommand {
        match subcommand.name.as_str() {
            "rebuild-index" => {
                match index::rebuild_index(&project_dir) {
                    Ok(_) => println!("Index rebuilt successfully."),
                    Err(e) => eprintln!("Error rebuilding index: {}", e),
                }
            }
            "reorder" => {
                let lane = subcommand.matches.args.get("lane")
                    .and_then(|a| a.value.as_str())
                    .unwrap_or_default();
                if lane.is_empty() {
                    eprintln!("Error: lane is required.");
                    return;
                }
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
            "list" => {
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
            "show" => {
                let id = subcommand.matches.args.get("id")
                    .and_then(|a| a.value.as_str())
                    .unwrap_or_default();
                if id.is_empty() {
                    eprintln!("Error: id is required.");
                    return;
                }
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
            "create" => {
                let title = subcommand.matches.args.get("title")
                    .and_then(|a| a.value.as_str())
                    .unwrap_or_default();
                if title.is_empty() {
                    eprintln!("Error: title is required.");
                    return;
                }
                let status = subcommand.matches.args.get("status").and_then(|a| a.value.as_str());
                let priority = subcommand.matches.args.get("priority").and_then(|a| a.value.as_str()).unwrap_or("medium");
                let epic = subcommand.matches.args.get("epic").and_then(|a| a.value.as_str()).map(|s| s.to_string());
                let tags_str = subcommand.matches.args.get("tags").and_then(|a| a.value.as_str()).unwrap_or("");
                let tags: Vec<String> = if tags_str.is_empty() { 
                    Vec::new() 
                } else { 
                    tags_str.split(',').map(|s| s.trim().to_string()).collect() 
                };

                match storage::read_board_config(&project_dir.join("board.yaml")) {
                    Ok(mut config) => {
                        let id = format!("{}-{:03}", config.prefix, config.next_id);
                        config.next_id += 1;
                        let _ = storage::write_board_config(&project_dir.join("board.yaml"), &config);
                        
                        let status_val = status.map(|s| s.to_string())
                            .unwrap_or_else(|| config.lanes.first().cloned().unwrap_or_else(|| "todo".to_string()));
                        
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
                                tags,
                                priority: priority.to_string(),
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
            "init" => {
                let name = subcommand.matches.args.get("name")
                    .and_then(|a| a.value.as_str())
                    .unwrap_or_default();
                let prefix = subcommand.matches.args.get("prefix")
                    .and_then(|a| a.value.as_str())
                    .unwrap_or_default();
                
                if name.is_empty() || prefix.is_empty() {
                    eprintln!("Error: name and prefix are required.");
                    return;
                }
                
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
                    return;
                }
                let _ = std::fs::create_dir_all(project_dir.join("tickets"));
                let _ = index::rebuild_index(&project_dir);
                println!("Initialized board '{}' with prefix '{}'.", name, prefix);
            }
            _ => {}
        }
    }
}
