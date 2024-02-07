use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Teacher {
    pub id: u64,
    pub index: u64,
    pub name: String,
}
