use crate::input::class::Class;
use crate::input::room::Room;
use super::aco_parameters::AcoParameters;

#[derive(Clone)]
struct Edge{
    to : [u64; 2],
    length : f64,
    pheromone : f64,
    heuristic : f64,
    next_pheromone : f64,
}

struct RoomPeriod{
    room : u64,
    period : u64,
}

pub struct Graph <'a>{
    edges: Vec<Vec<Vec<Edge>>>,
    num_of_classes: u64,
    num_of_rooms: u64, 
    num_of_periods: u64,
    room_periods: Vec<RoomPeriod>,
    parameters: &'a AcoParameters,
    classes: &'a Vec<Class>,
    rooms: &'a Vec<Room>,
}

impl<'a> Graph<'a>{
    pub fn new(parameters: &'a AcoParameters, classes: &'a Vec<Class>, rooms: &'a Vec<Room>) -> Graph<'a>{
        let num_of_classes = parameters.num_of_classes;
        let num_of_rooms = parameters.num_of_rooms;
        let num_of_periods = parameters.num_of_periods;
        let edges = 
            vec![
                vec![
                    vec![
                        Edge{to:[0,0], length:0.0, pheromone:0.0, heuristic:0.0, next_pheromone:0.0}; num_of_periods as usize
                        ]; num_of_rooms as usize
                    ]; num_of_classes as usize
                ];
        let room_periods = Vec::new();
        let mut res =  Graph{edges:edges, 
            num_of_classes:parameters.num_of_classes, 
            num_of_rooms:parameters.num_of_rooms, 
            num_of_periods:parameters.num_of_periods, 
            room_periods, parameters, classes, rooms};
        res.prepare_graph();
        return res;
    }
    pub fn get_max_pheromone(&self) -> f64{
        let mut max_pheromone = 0.0;
        for i in self.edges.iter(){
            for j in i.iter(){
                for k in j.iter(){
                    if k.pheromone > max_pheromone{
                        max_pheromone = k.pheromone;
                    }
                }
            }
        }
        max_pheromone
    }

    fn calc_edge_length(&self,p1:u64, p2:u64) -> f64{
        if p1 == p2{
            return 1 as f64;
        }
        if p1 > p2{
            return ((p1 - p2)+1) as f64;
        }
        return ((p2 - p1)+1) as f64;
    }

    fn prepare_graph(& mut self) {
        for i in 0..self.num_of_classes as usize {
            for j in 0..self.num_of_rooms as usize {
                for k in 0..self.num_of_periods as usize {
                    self.edges[i][j][k].to = [j as u64,k as u64];
                    self.edges[i][j][k].length = self.calc_edge_length(i as u64,j as u64);
                    self.edges[i][j][k].pheromone = self.parameters.q / self.edges[i][j][k].length;
                    self.edges[i][j][k].heuristic = self.parameters.q / self.edges[i][j][k].length;
                    self.edges[i][j][k].next_pheromone = 0.0;
                }
            }
        }
    }

    pub fn reset_graph(& mut self) {
        for i in 0..self.num_of_classes as usize{
            for j in 0..self.num_of_rooms as usize{
                for k in 0..self.num_of_periods as usize{
                    self.edges[i][j][k].next_pheromone = 0.0;
                }
            }
        }
    }
    pub fn reset_graph_when_stagnation(& mut self) {
        for i in 0..self.num_of_classes as usize{
            for j in 0..self.num_of_rooms as usize{
                for k in 0..self.num_of_periods as usize{
                    self.edges[i][j][k].pheromone = self.parameters.q / self.edges[i][j][k].length;
                }
            }
        }
    }
    pub fn get_pheromone(&self,class_index:usize,room_index:usize,period_index:usize) -> f64{
        return self.edges[class_index][room_index][period_index].pheromone;
    }
    pub fn get_class_ref(&self,class_index:usize) -> &Class{
        return &self.classes[class_index];
    }
    pub fn get_room_ref(&self,room_index:usize) -> &Room{
        return &self.rooms[room_index];
    }
    pub fn add_pheromone(& mut self,class_index:usize,room_index:usize,period_index:usize,pheromone:f64){
        self.edges[class_index][room_index][period_index].next_pheromone += pheromone;
    }
    pub fn set_edge_pheromone(& mut self,class_index:usize,room_index:usize,period_index:usize,pheromone:f64){
        self.edges[class_index][room_index][period_index].pheromone = pheromone;
    }
    pub fn get_next_pheromone(&self,class_index:usize,room_index:usize,period_index:usize) -> f64{
        return self.edges[class_index][room_index][period_index].next_pheromone;
    }
    pub fn get_edge(&self,class_index:usize,room_index:usize,period_index:usize) -> &Edge{
        return &self.edges[class_index][room_index][period_index];
    }
}