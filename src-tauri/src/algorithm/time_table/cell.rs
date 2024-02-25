
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Cell{
    ActiveCell(ActiveCell),
    BlankCell(BlankCell)
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActiveCell{
    pub id: usize,
    pub class_name: String,
    pub teachers: Option<Vec<String>>,
    pub students: Option<Vec<String>>,
    pub color: Option<String>,
    pub is_locked: Option<bool>,
    pub size: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlankCell{
    pub id: usize,
    pub size: Option<u64>,
}