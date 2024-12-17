// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::Mutex;
use tauri::Manager;
mod algorithm;
mod input;
use std::error::Error;
mod table_editor;
use algorithm::aco::aco_parameters::AcoParametersManager;
use algorithm::aco::aco_solver::ACOSolverManager;
use algorithm::time_table;
use input::InputManager;
use std::time::Instant;
use log::info;
use std::env;

#[tauri::command]
fn handle_adapt_input(
    input_manager: tauri::State<'_, InputManager>,
    solver_manager: tauri::State<'_, ACOSolverManager>,
    aco_parameters_manager: tauri::State<'_, AcoParametersManager>,
) -> Result<(), String> {
    info!("called handle_adapt_input");
    let input = input_manager.input.lock().unwrap();
    if let Some(input) = input.clone() {
        println!("adapt input to solver.");
        let parameters = algorithm::aco::aco_parameters::AcoParameters {
            num_of_ants: 3,
            num_of_classes: input.get_classes().len(),
            num_of_rooms: input.get_rooms().len(),
            num_of_periods: 5*5,
            num_of_day_lengths: 5,
            num_of_teachers: input.get_teachers().len(),
            num_of_students: input.get_student_groups().len(),
            size_of_frame: 1,
            q: 5.0,
            alpha: 1.0,
            beta: 2.0,
            rou: 0.5,
            max_iterations: 100,
            tau_min: 0.0015,
            tau_max: 100000.0,
            ant_prob_random: 0.001,
            super_not_change: 10000,
        };
        let solver = Some(algorithm::aco::aco_solver::ACOSolver {
            parameters: parameters.clone(),
            colony: algorithm::aco::colony::Colony::new(
                algorithm::aco::graph::Graph::new(
                    parameters.clone(),
                    input.get_classes().clone(),
                    input.get_rooms().clone(),
                    input.get_teachers().clone(),
                ),
                parameters.clone(),
            ),
            best_ant: None,
            super_ant: None,
            cnt_super_not_change: 0,
            input: input,
        });
        let mut manarged_solver = solver_manager.solver.lock().unwrap();
        manarged_solver.replace(solver.unwrap());
        let mut managed_parameters = aco_parameters_manager.parameters.lock().unwrap();
        managed_parameters.replace(parameters);
    } else {
        println!("no input!");
    }
    Ok(())
}
use input::handle_set_input;

#[tauri::command]
fn handle_aco_run_once(
    solver_manager: tauri::State<'_, ACOSolverManager>,
    timetable_manager: tauri::State<'_, time_table::TimeTableManager>,
) -> Result<time_table::TimeTable, String> {
    info!("called handle_aco_run_once");
    let mut managed_solver = solver_manager.solver.lock().unwrap();

    if let Some(solver) = managed_solver.as_mut() {
        let mut run_cnt = 0;
        let start = Instant::now();
        for _ in 0..10 {
            solver.run_aco_times(1);
            run_cnt += 1;
            if let Some(best_ant) = &solver.best_ant {
                println!("length:{}", best_ant.calc_all_path_length(solver.colony.get_graph()));
                if best_ant.calc_all_path_length(solver.colony.get_graph()) <= 0.5 {
                    break;
                }
            }
        }
        let duaration = start.elapsed();
        println!("times:{:?},{:?}", run_cnt, duaration);
        let res = time_table::convert_solver_to_timetable(solver).map_err(|e| e.to_string())?;
        time_table::save_timetable(timetable_manager, res.clone());
        /*
        println!(
            "violations_strict_student{:?}",
            solver.get_best_ant_same_group_violations_strictly()
        );
        println!(
            "violations_strict_teacher{:?}",
            solver.get_best_ant_same_teacher_violations_strictly()
        );
        println!(
            "violations_capacity{:?}",
            solver.get_best_ant_capacity_violations()
        );
        println!(
            "violations_strabble_days{:?}",
            solver.get_best_ant_strabble_days_violations()
        );
        */
        return Ok(res);
    }
    return Err("No ACOSolver".to_string());
}

#[tauri::command]
fn handle_aco_run_no_violations(
    solver_manager: tauri::State<'_, ACOSolverManager>,
    timetable_manager: tauri::State<'_, time_table::TimeTableManager>,
) -> Result<time_table::TimeTable, String> {
    info!("called handle_aco_run_no_violations");
    let mut managed_solver = solver_manager.solver.lock().unwrap();
    if let Some(solver) = managed_solver.as_mut() {
        let mut run_cnt = 0;
        let start = Instant::now();
        for _ in 0..2000 {
            solver.run_aco_times(1);
            run_cnt += 1;
            if let Some(best_ant) = &solver.best_ant {
                if best_ant.calc_all_path_length(solver.colony.get_graph()) <= 0.5 {
                    break;
                }
            println!("length:{}", best_ant.calc_all_path_length(solver.colony.get_graph()));
            }
        }
        let duaration = start.elapsed();
        println!("times:{:?},{:?}", run_cnt, duaration);
        let res = time_table::convert_solver_to_timetable(solver).map_err(|e| e.to_string())?;
        time_table::save_timetable(timetable_manager, res.clone());
        return Ok(res);
    }
    return Err("No ACOSolver".to_string());
}

#[tauri::command]
fn handle_calc_performance(
    input_manager: tauri::State<'_, InputManager>,
    aco_parameters_manager: tauri::State<'_, AcoParametersManager>,
    solver_manager: tauri::State<'_, ACOSolverManager>,
    timetable_manager: tauri::State<'_, time_table::TimeTableManager>,
) -> Result<(), String> {
    info!("called handle_calc_paformance");
    let mut times = Vec::<f64>::new();
    for _ in 0..100 {
        handle_adapt_input(input_manager.clone(), solver_manager.clone(), aco_parameters_manager.clone());
        let start = Instant::now();
        handle_aco_run_no_violations(solver_manager.clone(), timetable_manager.clone());
        let duration = start.elapsed();
        times.push(duration.as_secs_f64());
    }
    let average = times.iter().sum::<f64>() / times.len() as f64;
    for(i, time) in times.iter().enumerate() {
        println!("{}:{}", i, time);
    }
    println!("average:{}", average);
    
    return Err("No ACOSolver".to_string());
}
use algorithm::aco::aco_parameters::handle_get_periods;
use algorithm::aco::aco_solver::handle_one_hot_pheromone;
use algorithm::aco::aco_solver::handle_read_cells;
use input::handle_get_rooms;
use table_editor::handle_get_table;
use time_table::dump_timetable;
use time_table::handle_swap_cell;
use time_table::handle_switch_lock;
use time_table::is_swappable;
use time_table::load_timetable;
use time_table::handle_lock_no_violation;
use time_table::handle_unlock_violation;

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    //let input = input::Input::new();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            handle_adapt_input,
            handle_set_input,
            handle_aco_run_once,
            handle_aco_run_no_violations,
            handle_one_hot_pheromone,
            handle_get_table,
            handle_swap_cell,
            handle_read_cells,
            handle_switch_lock,
            is_swappable,
            handle_get_periods,
            handle_get_rooms,
            dump_timetable,
            load_timetable,
            handle_calc_performance,
            handle_lock_no_violation,
            handle_unlock_violation
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
            let aco_parameters_manager = AcoParametersManager {
                parameters: Mutex::new(None),
            };
            app.manage(aco_parameters_manager);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
