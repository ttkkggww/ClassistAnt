use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Cell {
    ActiveCell(ActiveCell),
    BlankCell(BlankCell),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActiveCell {
    pub id: usize,
    pub period: usize,
    pub room: usize,
    pub class_index: usize,
    pub class_name: String,
    pub teachers: Option<Vec<String>>,
    pub students: Option<Vec<String>>,
    pub color: Option<String>,
    pub is_locked: Option<bool>,
    pub size: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlankCell {
    pub id: usize,
    pub period: usize,
    pub room: usize,
    pub size: Option<u64>,
}
