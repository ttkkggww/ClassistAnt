// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::Manager;
mod input;
mod algorithm;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn handle_input(input:input::Input) -> Result<(),String> {
    println!("called handle_input");
    let parameters = algorithm::aco::aco_parameters::AcoParameters{
        num_of_ants: 5,
        num_of_classes: input.get_classes().len() as u64,
        num_of_rooms: input.get_rooms().len() as u64,
        num_of_periods: 5*5,
        num_of_teachers: input.get_teachers().len() as u64,
        num_of_students: input.get_student_groups().len() as u64,
        q: 1.0,
        alpha: 1.0,
        beta: 1.0,
        rou: 0.5,
        max_iterations: 100,
        tau_min: 0.0,
        tau_max: 100.0,
        ant_prob_random: 0.0,
        super_not_change: 100,
    };
    let mut graph = algorithm::aco::graph::Graph::new(&parameters, &input.get_classes(), &input.get_rooms());
    let mut colony = algorithm::aco::colony::Colony::new(&mut graph, &parameters);
    let mut solver = algorithm::aco::aco_solver::ACOSolver{
        parameters: &parameters,
        colony: &mut colony,
        best_ant_path: (1e9, Vec::new()),
        super_ant_path: (1e9, Vec::new()),
        cnt_super_not_change: 10000,
        input: &input,
    };
    println!("running aco");
    solver.run_aco();
    println!("aco finished");
    Ok(())
}

pub struct InputManager{
    input:Mutex<input::Input>,
}


#[tauri::command]
fn get_input(input_manager: tauri::State<'_,InputManager>) -> Result<(), String>{
    let input = input_manager.input.lock().unwrap();
    println!("{:?}",input.get_classes());
    Ok(())
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            handle_input
            ])
        .setup( |app| {
            let input = input::Input::new();
            let input_manager = InputManager{
                input: Mutex::new(input),
            };
            app.manage(input_manager);
            Ok(())
        }
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
