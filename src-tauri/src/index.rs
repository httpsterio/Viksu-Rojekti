use std::fs;
use std::path::Path;
use chrono::Local;
use crate::models::Index;
use crate::storage;

pub fn rebuild_index(dir: &Path) -> Result<Index, String> {
    let files = storage::list_card_files(dir)?;
    let mut cards = Vec::new();
    
    for file in files {
        let content = fs::read_to_string(&file)
            .map_err(|e| format!("Could not read card {}: {}", file.display(), e))?;
        let (meta, _) = storage::parse_card_file(&content)?;
        cards.push(meta);
    }
    
    let index = Index {
        generated: Local::now().to_rfc3339(),
        card_count: cards.len(),
        cards,
    };
    
    let yaml = serde_yaml::to_string(&index)
        .map_err(|e| format!("Index serialization error: {}", e))?;
        
    let index_path = dir.join("rojekti").join("rojekti.index.yaml");
    let tmp = index_path.with_extension("yaml.tmp");
    fs::write(&tmp, yaml)
        .map_err(|e| format!("Could not write index tmp file: {}", e))?;
    fs::rename(&tmp, &index_path)
        .map_err(|e| format!("Could not finalize index file: {}", e))?;
        
    Ok(index)
}
