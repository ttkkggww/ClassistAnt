use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Teacher {
    pub id: usize,
    pub index: usize,
    pub name: String,
    pub absent_days: Vec<usize>,
}
