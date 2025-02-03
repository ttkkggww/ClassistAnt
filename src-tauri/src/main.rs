// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::cmp::min;
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

fn reset_aco_solver(
    input: &input::Input,
    parameters: &algorithm::aco::aco_parameters::AcoParameters,
) -> algorithm::aco::aco_solver::ACOSolver {
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
        input: input.clone(),
        cnt: 0,
    });
    return solver.unwrap();
}

#[tauri::command]
fn handle_adapt_input(
    input_manager: tauri::State<'_, InputManager>,
    solver_manager: tauri::State<'_, ACOSolverManager>,
    aco_parameters_manager: tauri::State<'_, AcoParametersManager>,
) -> Result<(), String> {
    info!("called handle_adapt_input");
    let input = input_manager.input.lock().unwrap();
    if let Some(input) = input.clone() {
        let parameters = algorithm::aco::aco_parameters::AcoParameters {
            num_of_ants: 3,
            num_of_classes: input.get_classes().len(),
            num_of_rooms: input.get_rooms().len(),
            num_of_periods: 5*5,
            num_of_day_lengths: 5,
            num_of_teachers: input.get_teachers().len(),
            num_of_students: input.get_student_groups().len(),
            size_of_frame: 1,
            
            alpha: 1.0,// T 1.0 : K 2.0
            beta: 2.0, // T 2.0 : K 8.0
            q: 10.0,// T 10.0 : K 1.0
            rou:  0.5, // T 0.5 : K 0.95
            tau_min: 0.001,// T 0.001 : K 0.01
            tau_max: 100000.0, // T 100000.0 : K 10.0
            max_iterations: 100,
            ant_prob_random: 0.001,
            super_not_change: 10000,
        };
        let solver = reset_aco_solver(&input, &parameters);
        let mut manarged_solver = solver_manager.solver.lock().unwrap();
        manarged_solver.replace(solver);
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
                if best_ant.calc_all_path_length(solver.colony.get_graph()) <= 0.001 {
                    break;
                }
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
fn handle_aco_run_no_violations(
    solver_manager: tauri::State<'_, ACOSolverManager>,
    timetable_manager: tauri::State<'_, time_table::TimeTableManager>,
) -> Result<time_table::TimeTable, String> {
    info!("called handle_aco_run_no_violations");
    let mut managed_solver = solver_manager.solver.lock().unwrap();
    if let Some(solver) = managed_solver.as_mut() {
        let start = Instant::now();
        for gen in 0..2000 {
            solver.run_aco_times(1);
            if let Some(best_ant) = &solver.best_ant {
                if best_ant.calc_all_path_length(solver.colony.get_graph()) <= 0.001 {
                    break;
                }
                println!("{}", best_ant.count_violations(solver.colony.get_graph()));
            }
        }
        let duaration = start.elapsed();
        println!("{:?},{:?}", solver.cnt, duaration);
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
    let mut generations = Vec::<usize>::new();
    let mut is_solved = Vec::<bool>::new();
    let mut minimums = Vec::<usize>::new();
    for i in 0..50 {
        let mut is_cur_solve = false;
        if let Some(input) = input_manager.input.lock().unwrap().as_ref() {
            if let Some(parameters) = aco_parameters_manager.parameters.lock().unwrap().as_ref() {
                let mut solver = reset_aco_solver(input, parameters);
                let start = Instant::now();
                let mut minimum = 100000000;
                let mut count_cur_violation = 100000000;
                for _ in 0..2000 {
                        solver.run_aco_times(1);
                        if let Some(best_ant) = &solver.best_ant {
                            count_cur_violation = best_ant.count_violations(solver.colony.get_graph());
                            
                            if minimum > count_cur_violation {
                                minimum = count_cur_violation;
                            }
                            if count_cur_violation <= 0 {
                                break;
                            }
                    }
                }
                let duaration = start.elapsed();
                times.push(duaration.as_secs_f64());
                generations.push(solver.cnt);
                minimums.push(minimum);
                if let Some(best_ant) = &solver.best_ant {
                    if count_cur_violation <= 0 {
                        is_solved.push(true);
                        is_cur_solve = true;
                    } else {
                        is_solved.push(false);
                    }
                }
            }
        }
        println!("finished:{},{:},{}", i,is_cur_solve,minimums[i]);
    }
    let generation_average = generations.iter().sum::<usize>() as f64 / generations.len() as f64;
    let average = times.iter().sum::<f64>() / times.len() as f64;
    for(i, time) in times.iter().enumerate() {
        println!("{},{},{},{}", i, time,generations[i],minimums[i]);
    }
    println!("time_average:{}", average);
    println!("gen_average:{}", generation_average);
    
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
