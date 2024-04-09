
use tauri::Manager;
use std::sync::Mutex;
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

pub struct AcoParametersManager {
    pub parameters: Mutex<Option<AcoParameters>>,
}

static days_of_week : [&str;7]= ["月", "火", "水", "木", "金","土","日"];
#[tauri::command]
pub fn get_periods(parameters_manager: tauri::State<'_, AcoParametersManager>) -> Result<Vec<String>,String> {
    let parameters = parameters_manager.parameters.lock().unwrap();
    let mut res = Vec::new();
    if let Some(parameters) = &*parameters {
        for i in 0..parameters.num_of_periods {
            res.push(format!("{}曜日 {}限",days_of_week[(i/parameters.num_of_day_lengths)%days_of_week.len()],i%parameters.num_of_day_lengths));
        }
        return Ok(res);
    }
    Err("No parameters".to_string())
}