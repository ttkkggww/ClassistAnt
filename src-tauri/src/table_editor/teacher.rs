use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Teacher{
    pub id: String,
    pub name: String,
}