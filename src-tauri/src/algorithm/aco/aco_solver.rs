
use super::colony::Colony;
use super::aco_parameters::AcoParameters;
use crate::input::Input;

pub struct ACOSolver<'a>{
    pub parameters: &'a  AcoParameters,
    pub colony: &'a mut Colony<'a>,
    pub best_ant_path: (f64, Vec<[usize;2]>),
    pub super_ant_path: (f64, Vec<[usize;2]>),
    pub cnt_super_not_change: u64,
    pub input: &'a Input,
}


impl<'a> ACOSolver<'a>{

    pub fn run_aco(&mut self){
        for _ in 0..self.parameters.max_iterations{
            self.update_aco();
            println!("best path length: {}", self.best_ant_path.0);
        }

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
