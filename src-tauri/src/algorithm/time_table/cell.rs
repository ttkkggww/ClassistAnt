
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cell{
    pub class_name: String,
    pub teachers: Option<Vec<String>>,
    pub students: Option<Vec<String>>,
    pub color: Option<String>,
    pub is_locked: Option<bool>,
    pub size: Option<u64>,
}