use super::ant::Ant;
use super::aco_parameters::AcoParameters;
use super::graph::Graph;

#[derive(Clone)]
pub struct Colony{
    parameters:  AcoParameters,
    graph: Graph,
    ants: Vec<Ant>,
}

impl Colony{
    pub fn new(graph: Graph,parameters:AcoParameters)-> Colony{
        let mut ants = Vec::new();
        for _ in 0..parameters.num_of_ants{
            ants.push(Ant::new(parameters.clone()));
        }
        return Colony{parameters, graph, ants}
    }

    

    pub fn get_best_ant(&mut self)-> Ant{
        let mut best_ant = &self.ants[0];
        for ant in self.ants.iter(){
            if ant.calc_all_path_length(&self.graph) < best_ant.calc_all_path_length(&self.graph){
                best_ant = ant;
            }
        }
        return best_ant.clone();

    }

    pub fn update_colony(& mut self) {
        self.construct_ants();
        self.calc_next_pheromone();
    }
    
    pub fn reset_aco(&mut self){
        self.graph.reset_graph();
        self.reset_colony();
    }

    fn construct_ants(& mut self){
        for ant in self.ants.iter_mut(){
            ant.construct_path(&self.graph);
        }
    }
    fn calc_next_pheromone(&mut self){
        for ant in self.ants.iter_mut(){
            ant.update_next_pheromone(&mut self.graph);
        }
    }
    pub fn reset_colony(&mut self){
        for ant in self.ants.iter_mut(){
            ant.reset_ant();
        }
    }

    pub fn reset_pheromone(&mut self){
        self.graph.reset_graph_when_stagnation();
    }

    pub fn update_next_pheromone(&mut self){
        let num_of_classes = self.parameters.num_of_classes;
        let num_of_rooms = self.parameters.num_of_rooms;
        let num_of_periods = self.parameters.num_of_periods;
        let rou = self.parameters.rou;
        let tau_min = self.parameters.tau_min;
        let tau_max = self.parameters.tau_max;
        for i in 0..num_of_classes as usize{
            for j in 0..num_of_rooms as usize{
                for k in 0..num_of_periods as usize{
                    self.graph.set_pheromone(i, j, k, 
                        self.graph.get_pheromone(i, j, k)*rou
                        + self.graph.get_next_pheromone(i, j, k));
                    self.graph.set_pheromone(i, j, k, 
                        if self.graph.get_pheromone(i, j, k) < tau_min {
                            tau_min
                        } else if self.graph.get_pheromone(i, j, k) > tau_max {
                            tau_max
                        } else {
                            self.graph.get_pheromone(i, j, k)
                        }
                    );
                }
            }
        }
    }

    pub fn get_graph(&self) -> &Graph{
        &self.graph
    }

    pub fn set_one_hot_pheromone(&mut self, class_id: usize, room_id: usize, period_id: usize, min_pheromone:f64,max_pheromone:f64){
        self.graph.set_one_hot_pheromone(class_id, room_id, period_id, min_pheromone,max_pheromone);
    }

}