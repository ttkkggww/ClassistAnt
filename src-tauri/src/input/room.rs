use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Room {
    pub id: u64,
    pub index: u64,
    pub name: String,
    pub capacity: u64,
}

impl Room {
    pub fn get_capacity(&self) -> u64 {
        self.capacity
    }
}
