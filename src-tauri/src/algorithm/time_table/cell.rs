use serde::{Deserialize, Serialize};
use std::convert::AsMut;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Cell {
    ActiveCell(ActiveCell),
    BlankCell(BlankCell),
}

impl AsMut<Cell> for Cell {
    fn as_mut(&mut self) -> &mut Cell {
        return self;
    }
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
    pub size: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlankCell {
    pub id: usize,
    pub period: usize,
    pub room: usize,
    pub is_visible:bool,
    pub size: Option<usize>,
}
