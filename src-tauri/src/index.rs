use std::fs;
use std::path::Path;
use chrono::Local;
use crate::models::Index;
use crate::storage;

pub fn rebuild_index(dir: &Path) -> Result<Index, String> {
    let files = storage::list_ticket_files(dir)?;
    let mut tickets = Vec::new();
    
    for file in files {
        let content = fs::read_to_string(&file)
            .map_err(|e| format!("Could not read ticket {}: {}", file.display(), e))?;
        let (meta, _) = storage::parse_ticket_file(&content)?;
        tickets.push(meta);
    }
    
    let index = Index {
        generated: Local::now().to_rfc3339(),
        ticket_count: tickets.len(),
        tickets,
    };
    
    let yaml = serde_yaml::to_string(&index)
        .map_err(|e| format!("Index serialization error: {}", e))?;
        
    fs::write(dir.join("index.yaml"), yaml)
        .map_err(|e| format!("Could not write index.yaml: {}", e))?;
        
    Ok(index)
}
