use std::fs;
use std::path::{Path, PathBuf};
use crate::models::{BoardConfig, Card, CardMeta};

pub fn parse_card_file(content: &str) -> Result<(CardMeta, String), String> {
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err("Invalid frontmatter format: missing '---' delimiters".into());
    }
    let yaml = parts[1].trim();
    let body = parts[2].trim().to_string();
    
    let meta: CardMeta = serde_yaml::from_str(yaml)
        .map_err(|e| format!("YAML parse error: {}", e))?;
        
    Ok((meta, body))
}

pub fn serialize_card_file(meta: &CardMeta, body: &str) -> String {
    let yaml = serde_yaml::to_string(meta).unwrap_or_default();
    format!("---\n{}---\n\n{}\n", yaml, body)
}

pub fn read_board_config(path: &Path) -> Result<BoardConfig, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Could not read config file: {}", e))?;
    serde_yaml::from_str(&content)
        .map_err(|e| format!("YAML parse error in config file: {}", e))
}

pub fn write_board_config(path: &Path, config: &BoardConfig) -> Result<(), String> {
    let yaml = serde_yaml::to_string(config)
        .map_err(|e| format!("YAML serialization error: {}", e))?;
    let tmp = path.with_extension("yaml.tmp");
    fs::write(&tmp, yaml)
        .map_err(|e| format!("Could not write config tmp file: {}", e))?;
    fs::rename(&tmp, path)
        .map_err(|e| format!("Could not finalize config file: {}", e))
}

pub fn read_card(path: &Path) -> Result<Card, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Could not read card file: {}", e))?;
    let (meta, body) = parse_card_file(&content)?;
    Ok(Card { meta, body })
}

pub fn write_card(dir: &Path, card: &Card) -> Result<(), String> {
    let path = dir.join("rojekti").join("cards").join(format!("{}.md", card.meta.id));
    let content = serialize_card_file(&card.meta, &card.body);
    
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Could not create cards directory: {}", e))?;
    }
    
    let tmp = path.with_extension("md.tmp");
    fs::write(&tmp, content)
        .map_err(|e| format!("Could not write card tmp file: {}", e))?;
    fs::rename(&tmp, &path)
        .map_err(|e| format!("Could not finalize card file: {}", e))
}

pub fn delete_card_file(dir: &Path, id: &str) -> Result<(), String> {
    let path = dir.join("rojekti").join("cards").join(format!("{}.md", id));
    if path.exists() {
        let deleted_dir = dir.join("rojekti").join("deleted");
        fs::create_dir_all(&deleted_dir)
            .map_err(|e| format!("Could not create deleted directory: {}", e))?;
        
        let new_path = deleted_dir.join(format!("{}.md", id));
        fs::rename(path, new_path)
            .map_err(|e| format!("Could not move card to deleted directory: {}", e))?;
    }
    Ok(())
}

pub fn list_card_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let cards_dir = dir.join("rojekti").join("cards");
    if !cards_dir.exists() {
        return Ok(Vec::new());
    }
    
    let entries = fs::read_dir(cards_dir)
        .map_err(|e| format!("Could not read cards directory: {}", e))?;
        
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

pub fn read_all_cards(dir: &Path) -> Result<Vec<Card>, String> {
    let files = list_card_files(dir)?;
    let mut cards = Vec::new();
    for file in files {
        cards.push(read_card(&file)?);
    }
    Ok(cards)
}
