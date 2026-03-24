use std::fs;
use std::path::{Path, PathBuf};
use crate::models::{BoardConfig, BoardState, Card, CardMeta};

use uuid::Uuid;

pub fn ensure_ids(config: &mut BoardConfig) -> bool {
    let mut changed = false;

    for epic in config.epics.iter_mut() {
        if epic.id.is_empty() {
            epic.id = Uuid::new_v4().to_string();
            changed = true;
        }
    }

    for tag in config.tags.iter_mut() {
        if tag.id.is_empty() {
            tag.id = Uuid::new_v4().to_string();
            changed = true;
        }
    }

    for status in config.statuses.iter_mut() {
        if status.id.is_empty() {
            status.id = Uuid::new_v4().to_string();
            changed = true;
        }
    }

    changed
}

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

pub fn read_board_state(path: &Path) -> BoardState {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return BoardState::default(),
    };
    serde_yaml::from_str(&content).unwrap_or_default()
}

pub fn write_board_state(path: &Path, state: &BoardState) -> Result<(), String> {
    let yaml = serde_yaml::to_string(state)
        .map_err(|e| format!("YAML serialization error: {}", e))?;
    let tmp = path.with_extension("yaml.tmp");
    fs::write(&tmp, yaml)
        .map_err(|e| format!("Could not write state tmp file: {}", e))?;
    fs::rename(&tmp, path)
        .map_err(|e| format!("Could not finalize state file: {}", e))
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

pub fn rename_epic_or_tag(
    dir: &Path,
    config: &mut BoardConfig,
    is_epic: bool,
    index: usize,
    old_name: &str,
    new_name: &str,
    config_path: &Path,
) -> Result<(), String> {
    if is_epic {
        let entry = config.epics.get_mut(index)
            .ok_or_else(|| format!("Epic at index {} not found", index))?;
        entry.name = old_name.to_string();
        entry.pending_rename = Some(new_name.to_string());
    } else {
        let entry = config.tags.get_mut(index)
            .ok_or_else(|| format!("Tag at index {} not found", index))?;
        entry.name = old_name.to_string();
        entry.pending_rename = Some(new_name.to_string());
    }
    write_board_config(config_path, config)?;

    apply_pending_renames(dir, config, config_path)
}

pub fn apply_pending_renames(
    dir: &Path,
    config: &mut BoardConfig,
    config_path: &Path,
) -> Result<(), String> {
    let epic_renames: Vec<(String, String)> = config.epics.iter()
        .filter_map(|e| e.pending_rename.as_ref().map(|new| (e.name.clone(), new.clone())))
        .collect();
    let tag_renames: Vec<(String, String)> = config.tags.iter()
        .filter_map(|t| t.pending_rename.as_ref().map(|new| (t.name.clone(), new.clone())))
        .collect();

    if epic_renames.is_empty() && tag_renames.is_empty() {
        return Ok(());
    }

    let (cards, _) = read_all_cards(dir)?;
    for card in cards {
        let mut updated = card.clone();
        let mut changed = false;

        if let Some(ref epic) = card.meta.epic {
            if let Some((_, new)) = epic_renames.iter().find(|(old, _)| old == epic) {
                updated.meta.epic = Some(new.clone());
                changed = true;
            }
        }

        let new_tags: Vec<String> = card.meta.tags.iter().map(|t| {
            tag_renames.iter()
                .find(|(old, _)| old == t)
                .map(|(_, new)| new.clone())
                .unwrap_or_else(|| t.clone())
        }).collect();
        if new_tags != card.meta.tags {
            updated.meta.tags = new_tags;
            changed = true;
        }

        if changed {
            write_card(dir, &updated)?;
        }
    }

    for epic in config.epics.iter_mut() {
        if let Some(new_name) = epic.pending_rename.take() {
            epic.name = new_name;
        }
    }
    for tag in config.tags.iter_mut() {
        if let Some(new_name) = tag.pending_rename.take() {
            tag.name = new_name;
        }
    }
    write_board_config(config_path, config)
}

pub fn rename_status(
    dir: &Path,
    config: &mut BoardConfig,
    index: usize,
    old_name: &str,
    new_name: &str,
    config_path: &Path,
) -> Result<(), String> {
    let entry = config.statuses.get_mut(index)
        .ok_or_else(|| format!("Status at index {} not found", index))?;
    entry.name = old_name.to_string();
    entry.pending_rename = Some(new_name.to_string());
    write_board_config(config_path, config)?;

    apply_pending_status_renames(dir, config, config_path)
}

pub fn apply_pending_status_renames(
    dir: &Path,
    config: &mut BoardConfig,
    config_path: &Path,
) -> Result<(), String> {
    let renames: Vec<(String, String)> = config.statuses.iter()
        .filter_map(|s| s.pending_rename.as_ref().map(|new| (s.name.clone(), new.clone())))
        .collect();

    if renames.is_empty() {
        return Ok(());
    }

    let (cards, _) = read_all_cards(dir)?;
    for card in cards {
        if let Some((_, new)) = renames.iter().find(|(old, _)| old == &card.meta.status) {
            let mut updated = card.clone();
            updated.meta.status = new.clone();
            write_card(dir, &updated)?;
        }
    }

    for status in config.statuses.iter_mut() {
        if let Some(new_name) = status.pending_rename.take() {
            status.name = new_name;
        }
    }
    write_board_config(config_path, config)?;
    crate::index::rebuild_index(dir)?;
    Ok(())
}

pub fn read_all_cards(dir: &Path) -> Result<(Vec<Card>, Vec<String>), String> {
    let files = list_card_files(dir)?;
    let mut cards = Vec::new();
    let mut errors = Vec::new();

    for file in files {
        match read_card(&file) {
            Ok(card) => cards.push(card),
            Err(e) => {
                let filename = file.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                errors.push(format!("Could not load {}: {}", filename, e));
            }
        }
    }
    Ok((cards, errors))
}
