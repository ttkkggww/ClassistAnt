use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Room {
    pub id: usize,
    pub index: usize,
    pub name: String,
    pub capacity: usize,
}

impl Room {
    pub fn get_capacity(&self) -> usize {
        self.capacity
    }
}
