
use super::colony::Colony;
use super::aco_parameters::AcoParameters;
use crate::input::Input;

pub struct ACOSolver{
    pub parameters:  AcoParameters,
    pub colony:  Colony,
    pub best_ant_path: (f64, Vec<[usize;2]>),
    pub super_ant_path: (f64, Vec<[usize;2]>),
    pub cnt_super_not_change: u64,
    pub input: Input,
}


impl ACOSolver{

    pub fn run_aco(&mut self){
        for _ in 0..self.parameters.max_iterations{
            self.update_aco();
            println!("best path length: {}", self.best_ant_path.0);
        }

    }

    pub fn get_class_id_time_table(&self) -> Vec<Vec<i64>>{
        let mut res = vec![vec![-1; self.parameters.num_of_periods as usize]; self.parameters.num_of_rooms as usize];
        for(class_id, &[room_id, period_id]) in self.super_ant_path.1.iter().enumerate(){
            res[room_id as usize][period_id as usize] = class_id as i64;
        }
        res
    }

    pub fn get_pheromone_table(&self) -> Vec<Vec<f64>>{
        let mut res = vec![vec![0.0; self.parameters.num_of_periods as usize]; self.parameters.num_of_rooms as usize];
        for(class_id, &[room_id, period_id]) in self.super_ant_path.1.iter().enumerate(){
            res[room_id as usize][period_id as usize] = self.colony.get_graph().get_pheromone(class_id, room_id, period_id);
        }
        res
    }

    pub fn run_aco_times(&mut self, times: u64){
        for _ in 0..times{
            self.update_aco();
        }
    }

    pub fn get_super_ant_score(&self) -> f64{
        self.super_ant_path.0
    }

    fn update_aco(&mut self){
        self.update_colony();
        self.reset_aco();
        if self.super_ant_path.0 > self.best_ant_path.0{
            self.super_ant_path = self.best_ant_path.clone();
        }else{
            self.cnt_super_not_change+=1;
        }

        if self.cnt_super_not_change > self.parameters.super_not_change{
            self.colony.reset_pheromone();
            self.cnt_super_not_change = 0;
        }
    }


    fn update_colony(&mut self){
        self.colony.update_colony();
        self.update_next_pheromone();
        self.best_ant_path = self.colony.get_best_ant();
    }

    fn reset_aco(&mut self){
        self.colony.reset_aco();
    }
    fn update_next_pheromone(&mut self){
        self.colony.update_next_pheromone();
    }
}
