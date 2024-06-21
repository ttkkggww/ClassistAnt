use super::super::time_table::cell::Cell;
use super::aco_parameters::AcoParameters;
use super::ant::Ant;
use super::colony::Colony;
use super::graph::Graph;
use super::violations::Violations;
use crate::{algorithm::time_table::cell::ActiveCell, input::{class, Input}};
use std::sync::Mutex;
use tauri::Manager;

#[derive(Clone)]
pub struct ACOSolver {
    pub parameters: AcoParameters,
    pub colony: Colony,
    pub best_ant: Option<Ant>,
    pub super_ant: Option<Ant>,
    pub cnt_super_not_change: usize,
    pub input: Input,
}

impl ACOSolver {
    pub fn run_aco(&mut self, graph: &super::graph::Graph) {
        for _ in 0..self.parameters.max_iterations {
            self.update_aco();
            if let Some(best_ant) = &self.best_ant {
                println!("best path length: {}", best_ant.calc_all_path_length(graph));
            }
        }
    }

    pub fn get_class_index_time_table(&self) -> Vec<Vec<usize>> {
        let mut res = vec![
            vec![0; self.parameters.num_of_periods as usize];
            self.parameters.num_of_rooms as usize
        ];
        if let Some(ant) = &self.best_ant {
            for (class_id, &[room_id, period_id]) in ant.get_corresponding_crp().iter().enumerate() {
                for i in 0..self.input.get_classes()[class_id].serial_size {
                    res[room_id as usize][period_id as usize + i as usize] = class_id;
                }
            }
        }
        res
    }

    pub fn get_pheromone_table(&self) -> Vec<Vec<f64>> {
        let mut res = vec![
            vec![0.0; self.parameters.num_of_periods as usize];
            self.parameters.num_of_rooms as usize
        ];
        if let Some(ant) = &self.best_ant {
            for (class_id, &[room_id, period_id]) in ant.get_corresponding_crp().iter().enumerate()
            {
                res[room_id as usize][period_id as usize] = self
                    .colony
                    .get_graph()
                    .get_pheromone(class_id, room_id, period_id);
            }
        }
        res
    }
    pub fn run_aco_while_none_violation(&mut self) {
        self.update_aco();
        while self.get_best_ant_total_violations().len() > 0 {
            self.update_aco();
            self.cnt_super_not_change += 1;
        }
    }

    pub fn run_aco_times(&mut self, times: usize) {
        for _ in 0..times {
            self.update_aco();
        }
    }

    pub fn get_super_ant_score(&self) -> f64 {
        if let Some(ant) = &self.super_ant {
            return ant.calc_all_path_length(self.colony.get_graph());
        }
        return 0.0;
    }
    pub fn get_best_ant_score(&self) -> f64 {
        if let Some(ant) = &self.best_ant {
            return ant.calc_all_path_length(self.colony.get_graph());
        }
        return 0.0;
    }

    fn update_aco(&mut self) {
        self.update_colony();
        self.reset_aco();
        if let Some(best_ant) = &self.best_ant {
            if let Some(super_ant) = &self.super_ant {
                if best_ant.calc_all_path_length(self.colony.get_graph())
                    < super_ant.calc_all_path_length(self.colony.get_graph())
                {
                    self.super_ant = Some(best_ant.clone());
                } else {
                    self.cnt_super_not_change;
                }
            } else {
                self.super_ant = Some(best_ant.clone());
            }
            /*
            println!(
                "best path length: {}",
                best_ant.calc_all_path_length(self.colony.get_graph())
            );
            */
        }
        if self.cnt_super_not_change > self.parameters.super_not_change {
            println!("reset pheromone!");
            self.colony.reset_pheromone();
            self.cnt_super_not_change = 0;
        }
    }

    fn update_colony(&mut self) {
        self.colony.update_colony();
        self.update_next_pheromone();
        self.best_ant = Some(self.colony.get_best_ant());
    }

    fn reset_aco(&mut self) {
        self.colony.reset_aco();
    }
    fn update_next_pheromone(&mut self) {
        self.colony.update_next_pheromone();
    }

    pub fn get_parameters(&self) -> AcoParameters {
        self.parameters.clone()
    }
    pub fn get_best_ant(&self) -> Option<Ant> {
        return self.best_ant.clone();
    }
    pub fn get_best_ant_same_group_violations(&self) -> Vec<Violations> {
        if let Some(best_ant) = &self.best_ant {
            return best_ant.get_same_students_group_violations();
        }
        return Vec::new();
    }
    pub fn get_best_ant_same_teacher_violations(&self) -> Vec<Violations> {
        if let Some(best_ant) = &self.best_ant {
            return best_ant.get_same_teacher_violations();
        }
        return Vec::new();
    }
    pub fn get_best_ant_capacity_violations(&self) -> Vec<Violations> {
        if let Some(best_ant) = &self.best_ant {
            return best_ant.get_capacity_violations(self.colony.get_graph());
        }
        return Vec::new();
    }

    pub fn get_best_ant_total_violations(&self) -> Vec<Violations> {
        let mut res = Vec::new();
        res.append(&mut self.get_best_ant_same_group_violations());
        res.append(&mut self.get_best_ant_same_teacher_violations());
        res.append(&mut self.get_best_ant_capacity_violations());
        return res;
    }

    pub fn get_best_ant_same_teacher_violations_strictly(&self) -> Vec<Violations> {
        if let Some(best_ant) = &self.best_ant {
            return best_ant.get_same_teacher_violations_strictly(&self.input);
        }
        return Vec::new();
    }

    pub fn get_best_ant_same_group_violations_strictly(&self) -> Vec<Violations> {
        if let Some(best_ant) = &self.best_ant {
            return best_ant.get_same_students_group_violations_strictly(&self.input);
        }
        return Vec::new();
    }

    pub fn get_best_ant_strabble_days_violations(&self) -> Vec<Violations> {
        if let Some(best_ant) = &self.best_ant {
            return best_ant.get_strabble_days_violations(&self.input);
        }
        return Vec::new();
    }
    pub fn get_best_ant_total_violations_strictly(&self) -> Vec<Violations> {
        let mut res = Vec::new();
        res.append(&mut self.get_best_ant_same_group_violations());
        res.append(&mut self.get_best_ant_same_teacher_violations_strictly());
        res.append(&mut self.get_best_ant_capacity_violations());
        res
    }

    fn ceiling_max_pheromone(&self) -> f64 {
        return (self.parameters.num_of_ants as f64) * (self.parameters.q / self.parameters.rou);
    }

    pub fn set_one_hot_pheromone(&mut self, class_id: usize, room_id: usize, period_id: usize) {
        let max_pheromone = self.ceiling_max_pheromone();
        self.colony.set_one_hot_pheromone(
            class_id,
            room_id,
            period_id,
            self.parameters.tau_min,
            max_pheromone,
        );
    }
}

pub struct ACOSolverManager {
    pub solver: Mutex<Option<ACOSolver>>,
}

#[tauri::command]
pub fn handle_one_hot_pheromone(
    solver_manager: tauri::State<'_, ACOSolverManager>,
    class_id: usize,
    room_id: usize,
    period_id: usize,
) -> Result<(), String> {
    println!(
        "called handle_one_hot_pheromone {} {} {}",
        class_id, room_id, period_id
    );
    let mut managed_solver = solver_manager.solver.lock().unwrap();
    if let Some(solver) = managed_solver.as_mut() {
        solver.set_one_hot_pheromone(class_id, room_id, period_id);
    }
    Ok(())
}

#[tauri::command]
pub fn handle_read_cells(
    solver_manager: tauri::State<'_, ACOSolverManager>,
    cells: Vec<Option<ActiveCell>>,
) -> Result<(), String> {
    let mut managed_solver = solver_manager.solver.lock().unwrap();
    if let Some(solver) = managed_solver.as_mut() {
        solver.colony.get_graph_as_mut().load_cells(&cells);
        return Ok(());
    }
    return Err("solver is not initialized".to_string());
}
