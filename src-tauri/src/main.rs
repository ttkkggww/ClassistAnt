// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use algorithm::aco::violations::Violations;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time;
use tauri::Manager;
mod algorithm;
mod input;
use std::error::Error;
mod table_editor;
use algorithm::time_table;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn handle_input(input: input::Input) -> Result<(), String> {
    println!("called handle_input");
    let parameters = algorithm::aco::aco_parameters::AcoParameters {
        num_of_ants: 5,
        num_of_classes: input.get_classes().len() as usize,
        num_of_rooms: input.get_rooms().len() as usize,
        num_of_periods: 5 * 5,
        num_of_day_lengths: 5,
        num_of_teachers: input.get_teachers().len() as usize,
        num_of_students: input.get_student_groups().len() as usize,
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
    Ok(())
}

pub struct InputManager {
    input: Mutex<Option<input::Input>>,
}

use algorithm::aco::aco_solver::ACOSolverManager;

#[tauri::command]
fn handle_adapt_input(
    input_manager: tauri::State<'_, InputManager>,
    solver_manager: tauri::State<'_, ACOSolverManager>,
) -> Result<(), String> {
    let input = input_manager.input.lock().unwrap();
    if let Some(input) = input.clone() {
        println!("adapt input to solver.");
        let parameters = algorithm::aco::aco_parameters::AcoParameters {
            num_of_ants: 3,
            num_of_classes: input.get_classes().len(),
            num_of_rooms: input.get_rooms().len(),
            num_of_periods: 5 * 5,
            num_of_day_lengths: 5,
            num_of_teachers: input.get_teachers().len(),
            num_of_students: input.get_student_groups().len(),
            q: 10.0,
            alpha: 1.0,
            beta: 1.0,
            rou: 0.5,
            max_iterations: 100,
            tau_min: 0.001,
            tau_max: 100000.0,
            ant_prob_random: 0.0,
            super_not_change: 10000,
        };
        let solver = Some(algorithm::aco::aco_solver::ACOSolver {
            parameters: parameters.clone(),
            colony: algorithm::aco::colony::Colony::new(
                algorithm::aco::graph::Graph::new(
                    parameters.clone(),
                    input.get_classes().clone(),
                    input.get_rooms().clone(),
                ),
                parameters,
            ),
            best_ant: None,
            super_ant: None,
            cnt_super_not_change: 0,
            input: input,
        });
        let mut manarged_solver = solver_manager.solver.lock().unwrap();
        manarged_solver.replace(solver.unwrap());
    } else {
        println!("no input!");
    }
    Ok(())
}

#[tauri::command]
fn handle_set_input(input_manager: tauri::State<'_, InputManager>) -> Result<(), String> {
    println!("called handle_set_input");
    let input = input::Input::new();
    let mut managed_input = input_manager.input.lock().unwrap();
    *managed_input = Some(input);
    Ok(())
}

#[tauri::command]
fn handle_aco_run_once(
    solver_manager: tauri::State<'_, ACOSolverManager>,
    timetable_manager: tauri::State<'_, time_table::TimeTableManager>,
) -> Result<time_table::TimeTable, String> {
    let mut managed_solver = solver_manager.solver.lock().unwrap();

    if let Some(solver) = managed_solver.as_mut() {
        solver.run_aco_times(100);
        let res = time_table::convert_solver_to_timetable(solver).map_err(|e| e.to_string())?;
        time_table::save_timetable(timetable_manager, res.clone());
        println!("violations_strict_student{:?}",solver.get_best_ant_same_group_violations_strictly());
        println!("violations_strict_teacher{:?}",solver.get_best_ant_same_teacher_violations_strictly());
        println!("violations_capacity{:?}",solver.get_best_ant_capacity_violations());
        println!("violations_strabble_days{:?}",solver.get_best_ant_strabble_days_violations());
        return Ok(res);
    }
    return Err("No ACOSolver".to_string());
}

use algorithm::aco::aco_solver::handle_one_hot_pheromone;
use algorithm::aco::aco_solver::handle_read_cells;
use table_editor::handle_get_table;
use time_table::handle_swap_cell;
use time_table::handle_switch_lock;
use time_table::is_swappable;

fn main() -> Result<(), Box<dyn Error>> {
    //let input = input::Input::new();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            handle_input,
            handle_adapt_input,
            handle_set_input,
            handle_aco_run_once,
            handle_one_hot_pheromone,
            handle_get_table,
            handle_swap_cell,
            handle_read_cells,
            handle_switch_lock,
            is_swappable
        ])
        .setup(|app| {
            let input_manager = InputManager {
                input: Mutex::new(None),
            };
            app.manage(input_manager);
            let solver_manager = ACOSolverManager {
                solver: Mutex::new(None),
            };
            app.manage(solver_manager);
            let timetable_manager = time_table::TimeTableManager {
                timetable_manager: Mutex::new(None),
            };
            app.manage(timetable_manager);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
