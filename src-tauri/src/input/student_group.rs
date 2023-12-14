use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentGroup {
    id: u64,
    name: String,
    index: u64,
}