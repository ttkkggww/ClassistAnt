use super::graph::{Graph, self};
use super::aco_parameters::AcoParameters;
use crate::input::class::Class;
use std::collections::HashMap;
use rand::seq::SliceRandom;
use super::violations::{Violations, self};

static CAP_COEF:f64 = 2.0;
static TEACHER_COEF:f64 = 1.0;
static STUDENT_COEF:f64 = 1.0;

#[derive(Clone)]
pub struct Ant{
    visited_classes: Vec<bool>,
    visited_roomperiods:Vec<Vec<bool>>,
    corresponding_crp: Vec<[usize;2]>,
    parameters:  AcoParameters,
    teachers_times: Vec<HashMap<usize,Vec<usize>>>,
    students_times: Vec<HashMap<usize,Vec<usize>>>,
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
            for i in graph.get_class_ref(*v).get_teacher_indexes().iter(){
                if let Some(times) = self.teachers_times.get_mut(*i as usize){
                    if let Some(time) = times.get_mut(&to[1]){
                        time.push(to[0]);
                    }else{
                        times.insert(to[1],vec![to[0]]);
                    }
                }
            }
            for i in graph.get_class_ref(*v).get_students_group_indexes().iter(){
                if let Some(times) = self.students_times.get_mut(*i as usize){
                    if let Some(time) = times.get_mut(&to[1]){
                        time.push(to[0]);
                    }else{
                        times.insert(to[1],vec![to[0]]);
                    }
                }
            }
        }
    }

    pub fn update_next_pheromone(&mut self,graph: & mut Graph){
        let length_v = self.calc_all_path_length_par_period(graph);
        for i in 0..self.corresponding_crp.len(){
            let [room,period] = self.corresponding_crp[i];
            let q = self.parameters.q;
            graph.add_next_pheromone(i, room, period,q/length_v[period]);
        }
    }

    fn calc_all_path_length_par_period(&self,graph: &Graph) -> Vec<f64>{
        let mut length = vec![1.0; self.parameters.num_of_periods as usize];
        for class_id in 0..self.corresponding_crp.len(){
            let [room,period] = self.corresponding_crp[class_id];
            if graph.get_room_ref(room).get_capacity() < graph.get_class_ref(class_id).get_num_of_students(){
                length[period] += CAP_COEF;
            }
        }
        for mp in self.students_times.iter(){
            for (period,v) in mp.iter(){
                let ftime = (*v).len() as f64;
                length[*period] += (ftime*(ftime-1.0)/2.0 as f64)*STUDENT_COEF;
            }
        }
        for mp in self.teachers_times.iter(){
            for (period,v) in mp.iter(){
                let ftime = (*v).len() as f64;
                length[*period] += (ftime*(ftime-1.0)/2.0 as f64)*TEACHER_COEF;
            }
        }
        length
    }

    pub fn calc_all_path_length(&self,graph: & Graph) -> f64{
        let mut length = 1.0;
        for class_id in 0..self.corresponding_crp.len(){
            let [room,_] = self.corresponding_crp[class_id];
            if graph.get_room_ref(room).get_capacity() < graph.get_class_ref(class_id).get_num_of_students(){
                println!("capacity over:{:?} > {:?}",graph.get_class_ref(class_id).get_name(),room);
                length += CAP_COEF;
            }
        }
        let mut id = 0;
        for mp in self.students_times.iter(){
            for (period,v) in mp.iter(){
                let ftime = (*v).len() as f64;
                length += (ftime*(ftime-1.0)/2.0 as f64)*STUDENT_COEF;
                if (*v).len() > 1  {
                    println!("student over id:{:?},period:{:?}",id,period);
                }
            }
            id+=1;
        }
        id=0;
        for mp in self.teachers_times.iter(){
            for (period,v) in mp.iter(){
                let ftime = (*v).len() as f64;
                length += (ftime*(ftime-1.0)/2.0 as f64)*TEACHER_COEF;
                if (*v).len() > 1  {
                    println!("teacher over id:{:?},period:{:?}",id,period);
                }
            }
            id+=1;
        }
        println!("calc_all_length:{}",length);
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
                    graph.get_room_ref(room).get_capacity(), graph.get_class_ref(v), period as u64);
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

    fn calc_edge_length(&self, room_capacity: u64,
        class:&Class,period:u64) -> f64{
        let mut edge_length = 1.0;
        if class.get_num_of_students() > room_capacity{
            edge_length += CAP_COEF;
        }
        for id in class.get_students_group_indexes().iter(){
            if let Some(times) = self.students_times.get(*id as usize){
                if let Some(time) = times.get(&(period as usize)){
                    let ftime = (*time).len() as f64;
                    edge_length += (ftime*(ftime-1.0)/2.0 as f64)*STUDENT_COEF;
                }
            }
        }
        for id in class.get_teacher_indexes().iter(){
            if let Some(times) = self.teachers_times.get(*id as usize){
                if let Some(time) = times.get(&(period as usize)){
                    let ftime = (*time).len() as f64;
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

    pub fn get_same_teacher_violations(&self) -> Vec<Violations>{
        let mut res = Vec::new();
        for (_,mp) in (&self.teachers_times).iter().enumerate(){
            for (period_id, time) in mp{
                if time.len() > 1{
                    let violations = Violations::new(*period_id as u64, time.clone());
                    res.push(violations);
                }
            }
        }
        res
    }
    pub fn get_same_students_group_violations(&self) -> Vec<Violations>{
        let mut res = Vec::new();
        for (_,mp) in (&self.students_times).iter().enumerate(){
            for (period_id, time) in mp{
                if time.len() > 1{
                    let violations = Violations::new(*period_id as u64, time.clone());
                    res.push(violations);
                }
            }
        }
        res
    }

    pub fn get_capacity_violations(&self,graph : &Graph) -> Vec<Violations>{
        let mut res = Vec::new();
        for class_id in 0..self.corresponding_crp.len(){
            let [room,period] = self.corresponding_crp[class_id];
            if graph.get_room_ref(room).get_capacity() < graph.get_class_ref(class_id).get_num_of_students(){
                let mut v = Vec::new();
                v.push(class_id);
                let violations = Violations::new(period as u64, v);
                res.push(violations);
            }
        }
        res
    }

}