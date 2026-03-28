use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_rename: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    #[serde(default)]
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_rename: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Priority {
    pub name: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BoardConfig {
    pub name: String,
    pub prefix: String,
    pub next_id: u32,
    pub statuses: Vec<Status>,
    pub epics: Vec<Epic>,
    pub tags: Vec<Tag>,
    pub priorities: Vec<Priority>,
    #[serde(default)]
    pub done_statuses: Vec<String>,
    #[serde(default)]
    pub hidden_statuses: Vec<String>,
    #[serde(default)]
    pub hidden_statuses_enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Epic {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_rename: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChecklistItem {
    pub id: String,
    pub text: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CardMeta {
    pub id: String,
    pub title: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epic: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(deserialize_with = "deserialize_priority")]
    pub priority: u8,
    pub position: f64,
    pub created: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub checklist: Vec<ChecklistItem>,
}

fn deserialize_priority<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_yaml::Value::deserialize(deserializer)?;
    let n = match value {
        serde_yaml::Value::Number(n) => n.as_u64().unwrap_or(0),
        serde_yaml::Value::String(s) => s.trim().parse::<u64>().unwrap_or(0),
        _ => 0,
    };
    if n >= 1 && n <= 5 {
        Ok(n as u8)
    } else {
        Ok(0)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    #[serde(flatten)]
    pub meta: CardMeta,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AllCardsResult {
    pub cards: Vec<Card>,
    pub errors: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    pub generated: String,
    pub card_count: usize,
    pub cards: Vec<CardMeta>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActiveFilters {
    pub epic: Option<String>,
    pub tag: Option<String>,
    pub priority: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BoardState {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default)]
    pub collapsed_statuses: Vec<String>,
    #[serde(default)]
    pub active_filters: ActiveFilters,
    #[serde(default = "default_view")]
    pub view: String,
    #[serde(default)]
    pub window_width: u32,
    #[serde(default)]
    pub window_height: u32,
    #[serde(default)]
    pub window_x: i32,
    #[serde(default)]
    pub window_y: i32,
    #[serde(default)]
    pub show_hidden_lanes: bool,
}

fn default_theme() -> String { "light".to_string() }
fn default_view() -> String { "board".to_string() }

impl Default for BoardState {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            collapsed_statuses: Vec::new(),
            active_filters: ActiveFilters::default(),
            view: default_view(),
            window_width: 0,
            window_height: 0,
            window_x: 0,
            window_y: 0,
            show_hidden_lanes: false,
        }
    }
}

use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct AppState {
    pub project_dir: std::path::PathBuf,
    pub write_lock: Mutex<()>,
    pub last_gui_write: Arc<Mutex<Instant>>,
    pub watcher: Mutex<Option<notify::RecommendedWatcher>>,
}
