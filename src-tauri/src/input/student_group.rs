use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentGroup {
    pub id: usize,
    pub name: String,
    pub index: usize,
}
