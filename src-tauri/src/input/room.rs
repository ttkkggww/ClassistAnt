use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Room {
    id: u64,
    index: u64,
    name: String,
    capacity: u64,
}

impl Room{
    pub fn get_capacity(&self) -> u64{
        self.capacity
    }
}