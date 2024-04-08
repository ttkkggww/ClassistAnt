use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize,Debug)]
pub struct Violations {
    period: usize,
    rooms: Vec<usize>,
}

impl Violations {
    pub fn new(period: usize, rooms: Vec<usize>) -> Violations {
        Violations { period, rooms }
    }
}
