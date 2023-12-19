use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Violations{
    period:u64,
    rooms:Vec<usize>,
}

impl Violations {
    pub fn new(period:u64, rooms:Vec<usize>) -> Violations{
        Violations{period, rooms}
    }
}