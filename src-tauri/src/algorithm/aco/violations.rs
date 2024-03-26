use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Violations {
    period: usize,
    rooms: Vec<usize>,
}

impl Violations {
    pub fn new(period: usize, rooms: Vec<usize>) -> Violations {
        Violations { period, rooms }
    }
}
