
#[derive(Clone, Debug)]
pub struct AcoParameters {
    pub num_of_ants: u64,
    pub num_of_classes: u64,
    pub num_of_rooms: u64,
    pub num_of_periods: u64,
    pub num_of_teachers: u64,
    pub num_of_students: u64,
    pub q: f64,
    pub alpha: f64,
    pub beta: f64,
    pub rou: f64,
    pub max_iterations: u64,
    pub tau_min: f64,
    pub tau_max: f64,
    pub ant_prob_random: f64,
    pub super_not_change: u64,
}