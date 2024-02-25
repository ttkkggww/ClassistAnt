use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Class {
    pub id: String,
    pub name: String,
    pub teachers: String,
    pub candidate_rooms: String,
    pub student_groups: String,
    pub num_of_students: String,
}
