// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::Manager;
use serde::{Deserialize, Serialize};
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
    Ok(())
}

pub struct InputManager{
    input:Mutex<Option<input::Input>>,
}

pub struct ACOSolverManager{
    solver:Mutex<Option<algorithm::aco::aco_solver::ACOSolver>>,
}


#[tauri::command]
fn handle_adapt_input(input_manager: tauri::State<'_,InputManager>,solver_manager:tauri::State<'_,ACOSolverManager>) -> Result<(), String>{
    let input = input_manager.input.lock().unwrap();
    if let Some(input) = input.clone(){
        println!("adapt input to solver.");
        let parameters = algorithm::aco::aco_parameters::AcoParameters{
            num_of_ants: 3,
            num_of_classes: input.get_classes().len() as u64,
            num_of_rooms: input.get_rooms().len() as u64,
            num_of_periods: 5*5,
            num_of_teachers: input.get_teachers().len() as u64,
            num_of_students: input.get_student_groups().len() as u64,
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
        let solver = Some(algorithm::aco::aco_solver::ACOSolver{
            parameters: parameters.clone(),
            colony: algorithm::aco::colony::Colony::new( algorithm::aco::graph::Graph::new(parameters.clone(), input.get_classes().clone(), input.get_rooms().clone()), parameters),
            best_ant: None,
            super_ant: None,
            cnt_super_not_change: 0,
            input: input,
        });
        let mut manarged_solver = solver_manager.solver.lock().unwrap();
        manarged_solver.replace(solver.unwrap());
    }else{
        println!("no input!");
    }
    Ok(())
}

#[tauri::command]
fn handle_set_input(input_manager: tauri::State<'_,InputManager>, input:input::Input) -> Result<(), String>{
    println!("called handle_set_input");
    let mut managed_input = input_manager.input.lock().unwrap();
    *managed_input = Some(input);
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct TimeTable{
    cell_name:Vec<Vec<i64>>,
    pheromone_256:Vec<Vec<i64>>,
}

#[tauri::command]
fn handle_aco_run_once(solver_manager:tauri::State<'_,ACOSolverManager>) -> Result<TimeTable, String>{
    println!("called handle_aco_run_once");
    let mut managed_solver = solver_manager.solver.lock().unwrap();
    if let Some(solver) = managed_solver.as_mut(){
        solver.run_aco_times(10);
        let parameters = solver.get_parameters();
        let mut max_pheromone = parameters.q * parameters.num_of_ants as f64 / (1.0-parameters.rou);
        for class_id in 0..parameters.num_of_classes as usize{
            for room_id in 0..parameters.num_of_rooms as usize{
                for period_id in 0..parameters.num_of_periods as usize{
                    max_pheromone = max_pheromone.max(solver.colony.get_graph().get_pheromone(class_id, room_id, period_id));
                }
            }
        }
        let mut pheromone = vec![vec![0; parameters.num_of_periods as usize]; parameters.num_of_rooms as usize];
        if let Some(best_ant) = solver.get_best_ant(){
            for (class_id,&[room_id,period_id])in best_ant.get_corresponding_crp().iter().enumerate(){
                pheromone[room_id][period_id] = (solver.colony.get_graph().get_pheromone(class_id,room_id,period_id) / max_pheromone*255.0) as i64;
            }
        }
        println!("best path length: {}", solver.get_best_ant_score());
        //println!("pheromone: {:?}", pheromone);
        let res = TimeTable{
            cell_name:solver.get_class_id_time_table(),
            pheromone_256:pheromone,
        };
        return Ok(res);
    }
    return Err("No ACOSolver".to_string());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            handle_input,
            handle_adapt_input,
            handle_set_input,
            handle_aco_run_once,
            ])
        .setup( |app| {
            let input_manager = InputManager{
                input:Mutex::new(None),
            };
            app.manage(input_manager);
            let solver_manager = ACOSolverManager{
                solver:Mutex::new(None),
            };
            app.manage(solver_manager);
            Ok(())
        }
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
