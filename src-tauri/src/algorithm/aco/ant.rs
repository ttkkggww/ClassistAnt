use super::graph::Graph;
use super::aco_parameters::AcoParameters;
use crate::input::class::Class;
use std::collections::HashMap;
use rand::seq::SliceRandom;

static CAP_COEF:f64 = 5.0;
static TEACHER_COEF:f64 = 5.0;
static STUDENT_COEF:f64 = 3.0;

#[derive(Clone)]
pub struct Ant{
    visited_classes: Vec<bool>,
    visited_roomperiods:Vec<Vec<bool>>,
    corresponding_crp: Vec<[usize;2]>,
    parameters:  AcoParameters,
    teachers_times: Vec<HashMap<u64,u64>>,
    students_times: Vec<HashMap<u64,u64>>,
}

impl Ant{
    pub fn new(parameters: AcoParameters) -> Ant{
        let visited_classes = vec![false; parameters.num_of_classes as usize];
        let visited_roomperiods = vec![vec![false; parameters.num_of_periods as usize]; parameters.num_of_rooms as usize];
        let corresponding_crp = vec![[0,0]; parameters.num_of_classes as usize];
        let parameters = parameters;
        let teachers_times = vec![HashMap::new(); parameters.num_of_teachers as usize];
        let students_times = vec![HashMap::new(); parameters.num_of_students as usize];
        return Ant{ visited_classes, visited_roomperiods, corresponding_crp, parameters, teachers_times, students_times}
    }


    pub fn construct_path(& mut self,graph : &Graph){
        let shuffled_array = Ant::get_shuffled_array(self.parameters.num_of_classes);
        self.teachers_times = vec![HashMap::new(); self.parameters.num_of_teachers as usize];
        self.students_times = vec![HashMap::new(); self.parameters.num_of_students as usize];
        for v in shuffled_array.iter(){
            let (to_vertex, to_period) = self.calc_prob_from_v(*v,graph);
            let to:[usize;2];
            if rand::random::<f64>() < self.parameters.ant_prob_random {
                to = to_vertex[rand::random::<usize>() % to_vertex.len()];
            } else {
                let random_p = rand::random::<f64>();
                to = to_vertex[to_period.iter().position(|&x| x > random_p).unwrap()];
            }
            self.corresponding_crp[*v] = to;
            self.visited_classes[*v] = true;
            self.visited_roomperiods[to[0]][to[1]] = true;
        }
    }

    pub fn update_next_pheromone(&mut self,graph: & mut Graph){
        let length = self.calc_all_path_length(graph);
        for i in 0..self.corresponding_crp.len(){
            let [room,period] = self.corresponding_crp[i];
            let q = self.parameters.q;
            graph.add_pheromone(i, room, period,q/length);
        }
    }

    pub fn calc_all_path_length(&self,graph: & Graph) -> f64{
        let mut length = 1.0;
        for i in 0..self.corresponding_crp.len(){
            let [room,_] = self.corresponding_crp[i];
            if graph.get_room_ref(room).get_capacity() < graph.get_class_ref(i).get_num_of_students(){
                length += CAP_COEF;
            }
        }
        for mp in self.students_times.iter(){
            for (_,v) in mp.iter(){
                let ftime = *v as f64;
                length += (ftime*(ftime-1.0)/2.0 as f64)*STUDENT_COEF;
            }
        }
        for mp in self.teachers_times.iter(){
            for (_,v) in mp.iter(){
                let ftime = *v as f64;
                length += (ftime*(ftime-1.0)/2.0 as f64)*TEACHER_COEF;
            }
        }
        length
    }


    fn calc_prob_from_v(&self,v:usize,graph : &Graph) -> (Vec<[usize;2]>,Vec<f64>) {
        let mut sum_pheromone = 0.0;
        let mut to_vertexes = Vec::new();
        let mut to_pheromones = Vec::new();
        let alpha = self.parameters.alpha;
        let beta = self.parameters.beta;

        for room in 0..self.parameters.num_of_rooms as usize{
            for period in 0..self.parameters.num_of_periods as usize{
                if self.visited_roomperiods[room][period] == true{
                    continue
                }
                let pre_pheromone = graph.get_pheromone(v,room,period);
                let heuristics = self.parameters.q / self.calc_edge_length(
                    graph.get_class_ref(v).get_num_of_students(),
                    graph.get_room_ref(room).get_capacity()
                    , graph.get_class_ref(v), period as u64);
                let pheromone = pre_pheromone.powf(alpha) * heuristics.powf(beta);
                sum_pheromone += pheromone;
                to_vertexes.push([room,period]);
                to_pheromones.push(pheromone);
            }
        }
        let mut to_prob = to_pheromones.iter().map(|x| x/sum_pheromone).collect::<Vec<f64>>();
        for i in 1..to_prob.len(){
            to_prob[i] += to_prob[i-1];
        }
        (to_vertexes, to_prob)
    }

    fn calc_edge_length(&self,num_of_classs_students: u64, room_capacity: u64,
        class:&Class,period:u64) -> f64{
        let mut edge_length = 1.0;
        if num_of_classs_students > room_capacity{
            edge_length += CAP_COEF;
        }
        for id in class.get_students_group_indexes().iter(){
            if let Some(times) = self.students_times.get(*id as usize){
                if let Some(time) = times.get(&period){
                    let ftime = *time as f64;
                    edge_length += (ftime*(ftime-1.0)/2.0 as f64)*STUDENT_COEF;
                }
            }
        }
        for id in class.get_teacher_indexes().iter(){
            if let Some(times) = self.teachers_times.get(*id as usize){
                if let Some(time) = times.get(&period){
                    let ftime = *time as f64;
                    edge_length += (ftime*(ftime-1.0)/2.0 as f64)*TEACHER_COEF;
                }
            }
        }
        edge_length
    }

    fn get_shuffled_array(num_of_classes: u64) -> Vec<usize>{
        let mut array = Vec::new();
        for i in 0..num_of_classes as usize{
            array.push(i);
        }
        let mut rng = rand::thread_rng();
        array.shuffle(&mut rng);
        array
    }

    pub fn reset_ant(& mut self){
        self.visited_classes = vec![false; self.parameters.num_of_classes as usize];
        self.visited_roomperiods
            = vec![vec![false; self.parameters.num_of_periods as usize]; self.parameters.num_of_rooms as usize];
        self.corresponding_crp = vec![[0,0]; self.parameters.num_of_classes as usize];
    }

    pub fn get_corresponding_crp(&self) -> &Vec<[usize;2]>{
        &self.corresponding_crp
    }
}