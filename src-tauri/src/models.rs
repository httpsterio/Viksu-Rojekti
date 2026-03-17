use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BoardConfig {
    pub name: String,
    pub prefix: String,
    pub next_id: u32,
    pub lanes: Vec<String>,
    pub epics: Vec<Epic>,
    pub tags: Vec<String>,
    pub priorities: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Epic {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TicketMeta {
    pub id: String,
    pub title: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epic: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    pub priority: String,
    pub position: f64,
    pub created: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ticket {
    #[serde(flatten)]
    pub meta: TicketMeta,
    pub body: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    pub generated: String,
    pub ticket_count: usize,
    pub tickets: Vec<TicketMeta>,
}

pub struct AppState {
    pub project_dir: std::path::PathBuf,
}
