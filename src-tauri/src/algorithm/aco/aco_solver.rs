
use super::colony::Colony;
use super::aco_parameters::AcoParameters;
use super::ant::Ant;
use crate::input::Input;

pub struct ACOSolver{
    pub parameters:  AcoParameters,
    pub colony:  Colony,
    pub best_ant : Option<Ant>,
    pub super_ant :Option<Ant>,
    pub cnt_super_not_change: u64,
    pub input: Input,
}


impl ACOSolver{

    pub fn run_aco(&mut self, graph:  &super::graph::Graph){
        for _ in 0..self.parameters.max_iterations{
            self.update_aco();
            if let Some (best_ant) = &self.best_ant{
                println!("best path length: {}", best_ant.calc_all_path_length(graph));
            }
        }

    }

    pub fn get_class_id_time_table(&self) -> Vec<Vec<i64>>{
        let mut res = vec![vec![-1; self.parameters.num_of_periods as usize]; self.parameters.num_of_rooms as usize];
        if let Some(ant) = &self.best_ant{
            for(class_id, &[room_id, period_id]) in ant.get_corresponding_crp().iter().enumerate(){
                res[room_id as usize][period_id as usize] = class_id as i64;
            }
        }
        res
    }

    pub fn get_pheromone_table(&self) -> Vec<Vec<f64>>{
        let mut res = vec![vec![0.0; self.parameters.num_of_periods as usize]; self.parameters.num_of_rooms as usize];
        if let Some(ant) = &self.best_ant {
            for(class_id, &[room_id, period_id]) in ant.get_corresponding_crp().iter().enumerate(){
                res[room_id as usize][period_id as usize] = self.colony.get_graph().get_pheromone(class_id, room_id, period_id);
            }
        }
        res
    }

    pub fn run_aco_times(&mut self, times: u64){
        println!("start pheromone:{}",self.colony.get_graph().get_pheromone(0, 0, 0));
        for _ in 0..times{
            self.update_aco();
        }
        println!("end pheromone:{}",self.colony.get_graph().get_pheromone(0, 0, 0));
    }

    pub fn get_super_ant_score(&self) -> f64{
        if let Some(ant) = &self.super_ant{
            return ant.calc_all_path_length(self.colony.get_graph());
        }
        return 0.0;
    }
    pub fn get_best_ant_score(&self) -> f64{
        if let Some(ant) = &self.best_ant{
            return ant.calc_all_path_length(self.colony.get_graph());
        }
        return 0.0;
    }

    fn update_aco(&mut self){
        self.update_colony();
        self.reset_aco();
        if let Some(best_ant) = &self.best_ant{
            if let Some(super_ant) = &self.super_ant{
                if best_ant.calc_all_path_length(self.colony.get_graph()) < super_ant.calc_all_path_length(self.colony.get_graph()){
                    self.super_ant = Some(best_ant.clone());
                }else {
                    self.cnt_super_not_change;
                }
            }else{
                self.super_ant = Some(best_ant.clone());
            }
        }
        if self.cnt_super_not_change > self.parameters.super_not_change{
            println!("reset pheromone!");
            self.colony.reset_pheromone();
            self.cnt_super_not_change = 0;
        }
    }


    fn update_colony(&mut self){
        self.colony.update_colony();
        self.update_next_pheromone();
        self.best_ant = Some(self.colony.get_best_ant());
    }

    fn reset_aco(&mut self){
        self.colony.reset_aco();
    }
    fn update_next_pheromone(&mut self){
        self.colony.update_next_pheromone();
    }

    pub fn get_parameters(&self) -> AcoParameters{
        self.parameters.clone()
    }
    pub fn get_best_ant(&self) -> Option<Ant>{
        return self.best_ant.clone();
    }
    
}
