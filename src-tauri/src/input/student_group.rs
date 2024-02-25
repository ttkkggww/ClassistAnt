use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentGroup {
    pub id: u64,
    pub name: String,
    pub index: u64,
}
