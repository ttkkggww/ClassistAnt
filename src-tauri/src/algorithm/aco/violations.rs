use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Violations {
    pub period: usize,
    pub rooms: Vec<usize>,
}

impl Violations {
    pub fn new(period: usize, rooms: Vec<usize>) -> Violations {
        Violations { period, rooms }
    }
}

#[derive(Serialize, Deserialize, Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub struct  CellsViolation {
    pub is_violated:bool,
    pub same_student_same_time:Vec<Violations>,
    pub same_teacher_same_time:Vec<Violations>,
    pub capacity_over:Vec<Violations>,
    pub strabble_days:Vec<Violations>,
}
