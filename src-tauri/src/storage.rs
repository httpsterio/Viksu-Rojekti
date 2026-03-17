use std::fs;
use std::path::{Path, PathBuf};
use crate::models::{BoardConfig, Ticket, TicketMeta};

pub fn parse_ticket_file(content: &str) -> Result<(TicketMeta, String), String> {
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err("Invalid frontmatter format: missing '---' delimiters".into());
    }
    let yaml = parts[1].trim();
    let body = parts[2].trim().to_string();
    
    let meta: TicketMeta = serde_yaml::from_str(yaml)
        .map_err(|e| format!("YAML parse error: {}", e))?;
        
    Ok((meta, body))
}

pub fn serialize_ticket_file(meta: &TicketMeta, body: &str) -> String {
    let yaml = serde_yaml::to_string(meta).unwrap_or_default();
    format!("---\n{}---\n\n{}\n", yaml, body)
}

pub fn read_board_config(path: &Path) -> Result<BoardConfig, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Could not read board.yaml: {}", e))?;
    serde_yaml::from_str(&content)
        .map_err(|e| format!("YAML parse error in board.yaml: {}", e))
}

pub fn write_board_config(path: &Path, config: &BoardConfig) -> Result<(), String> {
    let yaml = serde_yaml::to_string(config)
        .map_err(|e| format!("YAML serialization error: {}", e))?;
    fs::write(path, yaml)
        .map_err(|e| format!("Could not write board.yaml: {}", e))
}

pub fn read_ticket(path: &Path) -> Result<Ticket, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Could not read ticket file: {}", e))?;
    let (meta, body) = parse_ticket_file(&content)?;
    Ok(Ticket { meta, body })
}

pub fn write_ticket(dir: &Path, ticket: &Ticket) -> Result<(), String> {
    let path = dir.join("tickets").join(format!("{}.md", ticket.meta.id));
    let content = serialize_ticket_file(&ticket.meta, &ticket.body);
    
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Could not create tickets directory: {}", e))?;
    }
    
    fs::write(path, content)
        .map_err(|e| format!("Could not write ticket file: {}", e))
}

pub fn delete_ticket_file(dir: &Path, id: &str) -> Result<(), String> {
    let path = dir.join("tickets").join(format!("{}.md", id));
    if path.exists() {
        fs::remove_file(path)
            .map_err(|e| format!("Could not delete ticket file: {}", e))?;
    }
    Ok(())
}

pub fn list_ticket_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let tickets_dir = dir.join("tickets");
    if !tickets_dir.exists() {
        return Ok(Vec::new());
    }
    
    let entries = fs::read_dir(tickets_dir)
        .map_err(|e| format!("Could not read tickets directory: {}", e))?;
        
    let mut files = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("Read dir error: {}", e))?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "md") {
            files.push(path);
        }
    }
    Ok(files)
}

pub fn read_all_tickets(dir: &Path) -> Result<Vec<Ticket>, String> {
    let files = list_ticket_files(dir)?;
    let mut tickets = Vec::new();
    for file in files {
        tickets.push(read_ticket(&file)?);
    }
    Ok(tickets)
}
