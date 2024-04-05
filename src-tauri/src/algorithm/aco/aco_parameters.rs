#[derive(Clone, Debug)]
pub struct AcoParameters {
    pub num_of_ants: usize,
    pub num_of_classes: usize,
    pub num_of_rooms: usize,
    pub num_of_periods: usize,
    pub num_of_day_lengths: usize,
    pub num_of_teachers: usize,
    pub num_of_students: usize,
    pub q: f64,
    pub alpha: f64,
    pub beta: f64,
    pub rou: f64,
    pub max_iterations: usize,
    pub tau_min: f64,
    pub tau_max: f64,
    pub ant_prob_random: f64,
    pub super_not_change: usize,
}
