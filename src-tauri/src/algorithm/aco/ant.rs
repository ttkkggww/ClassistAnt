use super::aco_parameters::AcoParameters;
use super::graph::{self, Graph};
use super::violations::{self, Violations};
use crate::input::class::{self, Class};
use crate::input::room::Room;

use crate::input::teacher::Teacher;
use crate::input::Input;
use crate::table_editor::Teachers;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::collections::{BTreeMap, BTreeSet};
use std::vec;

static CAP_COEF: f64 = 2.0;
static TEACHER_COEF: f64 = 5.0;
static STUDENT_COEF: f64 = 3.0;
static ABSENT_DAYS_COEF: f64 = 3.0;
static STRADDLE_DAYS_COEF: f64 = 1.0;
static COLLECTION_COEF: f64 = 1.0;
static SEQUENTIAL_FROM_START_COEF: f64 = 4.0;

#[derive(Clone)]
pub struct Ant {
    visited_classes: Vec<bool>,
    visited_roomperiods: Vec<Vec<bool>>,
    corresponding_crp: Vec<[usize; 2]>,
    parameters: AcoParameters,
    //teachers_times[teacher_id][period] = [room_id, room_id, ...]gg
    work_periods_each_teachers: Vec<BTreeMap<usize, Vec<usize>>>,
    //teachers_times[teacher_id][period] = [room_id, room_id, ...]
    work_periods_each_students: Vec<BTreeMap<usize, Vec<usize>>>,
}

impl Ant {
    pub fn new(parameters: AcoParameters) -> Ant {
        let visited_classes = vec![false; parameters.num_of_classes as usize];
        let visited_roomperiods =
            vec![vec![false; parameters.num_of_periods as usize]; parameters.num_of_rooms as usize];
        let corresponding_crp = vec![[0, 0]; parameters.num_of_classes as usize];
        let parameters = parameters;
        let teachers_times = vec![BTreeMap::new(); parameters.num_of_teachers as usize];
        let students_times = vec![BTreeMap::new(); parameters.num_of_students as usize];
        return Ant {
            visited_classes,
            visited_roomperiods,
            corresponding_crp,
            parameters,
            work_periods_each_teachers: teachers_times,
            work_periods_each_students: students_times,
        };
    }

    fn allocate_classes(
        &mut self,
        class_index: usize,
        room_index: usize,
        period_index: usize,
        graph: &Graph,
    ) {
        let serial_size = graph.get_class(class_index).serial_size;
        self.corresponding_crp[class_index] = [room_index, period_index];
        self.visited_classes[class_index] = true;
        for i in 0..serial_size {
            self.visited_roomperiods[room_index][period_index + i] = true;
        }
        //get teacher indexes in classes,then add time;
        for teacher_index in graph
            .get_class_ref(class_index)
            .get_teacher_indexes()
            .iter()
        {
            if let Some(times) = self
                .work_periods_each_teachers
                .get_mut(*teacher_index as usize)
            {
                for i in 0..serial_size {
                    if let Some(time) = times.get_mut(&(period_index + i)) {
                        time.push(room_index);
                    } else {
                        times.insert(period_index + i, vec![room_index]);
                    }
                }
            }
        }
        //get student group indexes in classes,then add time;
        for student_index in graph
            .get_class_ref(class_index)
            .get_students_group_indexes()
            .iter()
        {
            if let Some(times) = self
                .work_periods_each_students
                .get_mut(*student_index as usize)
            {
                for i in 0..serial_size {
                    if let Some(time) = times.get_mut(&(period_index + i)) {
                        time.push(room_index);
                    } else {
                        times.insert(period_index + i, vec![room_index]);
                    }
                }
            }
        }
    }

    pub fn construct_path(&mut self, graph: &Graph) {
        let shuffled_array = Ant::get_shuffled_array(self.parameters.num_of_classes);
        self.work_periods_each_teachers =
            vec![BTreeMap::new(); self.parameters.num_of_teachers as usize];
        self.work_periods_each_students =
            vec![BTreeMap::new(); self.parameters.num_of_students as usize];
        //preallocate locked classes
        for v in shuffled_array.iter() {
            if let Some(to) = graph.get_classes_is_locked(*v) {
                self.allocate_classes(*v, to.0, to.1, graph);
            }
        }
        //allocate with pheromone
        for v in shuffled_array.iter() {
            if self.visited_classes[*v] {
                continue;
            }
            let (to_vertex, to_period) = self.calc_prob_from_v(*v, graph);
            let to: [usize; 2];
            if rand::random::<f64>() < self.parameters.ant_prob_random {
                to = to_vertex[rand::random::<usize>() % to_vertex.len()];
            } else {
                let random_p = rand::random::<f64>();
                to = to_vertex[to_period.iter().position(|&x| x > random_p).unwrap()];
            }
            self.allocate_classes(*v, to[0], to[1], graph);
        }
    }

    fn calc_all_path_length_each_frame(&self) -> Vec<Vec<f64>> {
        let mut length =
            vec![
                vec![1.0; self.parameters.num_of_periods / self.parameters.size_of_frame];
                self.parameters.num_of_rooms
            ];
        for r in 0..self.parameters.num_of_rooms as usize {
            for f in 0..self.parameters.num_of_periods / self.parameters.size_of_frame {
                let mut is_empty = true;
                let mut count_in_frame = 0;
                for p in f * self.parameters.size_of_frame..(f + 1) * self.parameters.size_of_frame
                {
                    if !self.visited_roomperiods[r][p] {
                    } else {
                        count_in_frame += 1;
                        is_empty = false;
                    }
                }
                if count_in_frame != self.parameters.size_of_frame {
                    length[r][f] +=
                        ((self.parameters.size_of_frame - count_in_frame) as f64) * COLLECTION_COEF;
                }
                if is_empty {
                    length[r][f] = 1.0;
                }
            }
        }
        length
    }

    pub fn update_next_pheromone(&mut self, graph: &mut Graph) {
        let length_period = self.calc_all_path_length_each_period(graph);
        let length_room = self.calc_all_path_length_each_room(graph);
        let length_frame = self.calc_all_path_length_each_frame();
        for i in 0..self.corresponding_crp.len() {
            let [room, period] = self.corresponding_crp[i];
            let q = self.parameters.q;
            graph.add_next_pheromone(
                i,
                room,
                period,
                q / (length_period[period]
                    + length_room[room]
                    + length_frame[room][period / self.parameters.size_of_frame]
                    - 1.0
                    - 1.0),
            );
        }
    }

    // capacity ,students and teachers
    fn calc_all_path_length_each_period(&self, graph: &Graph) -> Vec<f64> {
        let mut length = vec![1.0; self.parameters.num_of_periods as usize];
        for class_id in 0..self.corresponding_crp.len() {
            let [room, period] = self.corresponding_crp[class_id];
            if graph.get_room_ref(room).get_capacity()
                < graph.get_class_ref(class_id).get_num_of_students()
            {
                length[period] += CAP_COEF;
            }
        }
        for mp in self.work_periods_each_students.iter() {
            for (period, v) in mp.iter() {
                let ftime = (*v).len() as f64;
                length[*period] += (ftime * (ftime - 1.0) / 2.0 as f64) * STUDENT_COEF;
            }
        }
        for mp in self.work_periods_each_teachers.iter() {
            for (period, v) in mp.iter() {
                let ftime = (*v).len() as f64;
                length[*period] += (ftime * (ftime - 1.0) / 2.0 as f64) * TEACHER_COEF;
            }
        }
        for class_id in 0..self.corresponding_crp.len() {
            let [room, period] = self.corresponding_crp[class_id];
            let absent_days = self.calc_absent_days(
                &graph.get_class_ref(class_id).get_teacher_indexes(),
                graph.get_teachers_ref(),
            );
            if (absent_days.contains(&period)) {
                length[period] += ABSENT_DAYS_COEF;
            }
        }
        length
    }

    // straddle days
    fn calc_all_path_length_each_room(&self, graph: &Graph) -> Vec<f64> {
        let mut length = vec![1.0; self.parameters.num_of_rooms as usize];
        for class_id in 0..self.corresponding_crp.len() {
            let [room, period] = self.corresponding_crp[class_id];
            let serial_size = graph.get_class(class_id).serial_size;
            if (period % self.parameters.num_of_day_lengths) + serial_size
                > self.parameters.num_of_day_lengths
            {
                length[room] += STRADDLE_DAYS_COEF;
            }
        }
        length
    }

    pub fn calc_all_path_length(&self, graph: &Graph) -> f64 {
        let mut length = 1.0;
        let length_period = self.calc_all_path_length_each_period(graph);
        let length_room = self.calc_all_path_length_each_room(graph);
        let length_frame = self.calc_all_path_length_each_frame();
        for p in &length_period {
            length += p - 1.0;
        }
        for r in &length_room {
            length += r - 1.0;
        }
        for p in &length_frame {
            for f in p {
                length += f - 1.0;
            }
        }

        length
    }

    fn calc_allocatable_room_periods(&self, serial_size: usize, graph: &Graph) -> Vec<[usize; 2]> {
        let mut res = Vec::new();
        for room in 0..self.parameters.num_of_rooms as usize {
            for period in 0..(self.parameters.num_of_periods - serial_size + 1) as usize {
                let mut is_allocatable = true;
                for i in 0..serial_size {
                    if self.visited_roomperiods[room][period + i] == true {
                        is_allocatable = false;
                        break;
                    }
                }
                if is_allocatable {
                    res.push([room, period]);
                }
            }
        }
        res
    }

    fn calc_absent_days(
        &self,
        teacher_indices: &Vec<usize>,
        teachers: &Vec<Teacher>,
    ) -> BTreeSet<usize> {
        let mut res = BTreeSet::new();
        for teacher_id in teacher_indices.iter() {
            res.extend(teachers[*teacher_id].absent_days.clone());
        }
        res
    }

    fn calc_prob_from_v(&self, v: usize, graph: &Graph) -> (Vec<[usize; 2]>, Vec<f64>) {
        let mut sum_value = 0.0;
        let mut to_vertexes = Vec::new();
        let mut pre_normalized_values = Vec::new();
        let alpha = self.parameters.alpha;
        let beta = self.parameters.beta;
        let class_period_length = graph.get_class(v).serial_size;

        for [room, period] in self.calc_allocatable_room_periods(class_period_length, graph) {
            let pheromone = graph.get_pheromone(v, room, period);

            let heuristics = self.parameters.q
                / self.calc_edge_length(
                    graph.get_room_ref(room),
                    graph.get_class_ref(v),
                    &self.calc_absent_days(
                        &graph.get_class_ref(v).get_teacher_indexes(),
                        graph.get_teachers_ref(),
                    ),
                    period as usize,
                    self.parameters.size_of_frame,
                );
            let pre_normalized_value = pheromone.powf(alpha) * heuristics.powf(beta);
            if v == 0 {}
            sum_value += pre_normalized_value;
            to_vertexes.push([room, period]);
            pre_normalized_values.push(pre_normalized_value);
        }
        let mut to_prob = pre_normalized_values
            .iter()
            .map(|x| x / sum_value)
            .collect::<Vec<f64>>();
        for i in 1..to_prob.len() {
            to_prob[i] += to_prob[i - 1];
        }
        (to_vertexes, to_prob)
    }

    pub fn calc_prob_from_v_igunore_visited(
        &self,
        v: usize,
        graph: &Graph,
    ) -> (Vec<[usize; 2]>, Vec<f64>) {
        let mut sum_pheromone = 0.0;
        let mut to_vertexes = Vec::new();
        let mut to_pheromones = Vec::new();
        let alpha = self.parameters.alpha;
        let beta = self.parameters.beta;

        for room in 0..self.parameters.num_of_rooms as usize {
            for period in 0..self.parameters.num_of_periods as usize {
                let pre_pheromone = graph.get_pheromone(v, room, period);
                let heuristics = self.parameters.q
                    / self.calc_edge_length(
                        graph.get_room_ref(room),
                        graph.get_class_ref(v),
                        &self.calc_absent_days(
                            &graph.get_class_ref(v).get_teacher_indexes(),
                            graph.get_teachers_ref(),
                        ),
                        period as usize,
                        self.parameters.size_of_frame,
                    );
                let pheromone = pre_pheromone.powf(alpha) * heuristics.powf(beta);
                if v == 0 {}
                sum_pheromone += pheromone;
                to_vertexes.push([room, period]);
                to_pheromones.push(pheromone);
            }
        }
        let to_prob = to_pheromones
            .iter()
            .map(|x| x / sum_pheromone)
            .collect::<Vec<f64>>();
        (to_vertexes, to_prob)
    }

    fn calc_edge_length(
        &self,
        room: &Room,
        class: &Class,
        absent_days: &BTreeSet<usize>,
        period: usize,
        size_of_frame: usize,
    ) -> f64 {
        let mut edge_length = 1.0;
        //capacity violation
        if class.get_num_of_students() > room.get_capacity() {
            edge_length += CAP_COEF;
        }
        //students violation
        for id in class.get_students_group_indexes().iter() {
            if let Some(times) = self.work_periods_each_students.get(*id as usize) {
                if let Some(time) = times.get(&(period as usize)) {
                    let ftime = (*time).len() as f64;
                    edge_length += (ftime * (ftime - 1.0) / 2.0 as f64) * STUDENT_COEF;
                }
            }
        }
        //teachers violation
        for id in class.get_teacher_indexes().iter() {
            if let Some(times) = self.work_periods_each_teachers.get(*id as usize) {
                if let Some(time) = times.get(&(period as usize)) {
                    let ftime = (*time).len() as f64;
                    edge_length += (ftime * (ftime - 1.0) / 2.0 as f64) * TEACHER_COEF;
                }
            }
        }
        //teacher abset days violation
        if absent_days.contains(&period) {
            edge_length += ABSENT_DAYS_COEF;
        }
        //straddle days violation
        if (period % self.parameters.num_of_day_lengths) + class.serial_size
            > self.parameters.num_of_day_lengths
        {
            edge_length += STRADDLE_DAYS_COEF;
        }
        //collection violation
        {
            let start_frame = period / size_of_frame * size_of_frame;
            let mut count_in_frame = 0;
            for p in start_frame..(start_frame + size_of_frame) {
                if self.visited_roomperiods[room.index][p] {
                    count_in_frame += 1;
                }
            }
            edge_length += (size_of_frame - count_in_frame) as f64 * COLLECTION_COEF;
            if count_in_frame == 0 {
                edge_length += (size_of_frame) as f64 * COLLECTION_COEF;
            }
        }
        edge_length
    }

    fn get_shuffled_array(num_of_classes: usize) -> Vec<usize> {
        let mut array = Vec::new();
        for i in 0..num_of_classes as usize {
            array.push(i);
        }
        let mut rng = rand::thread_rng();
        array.shuffle(&mut rng);
        array
    }

    pub fn reset_ant(&mut self) {
        self.visited_classes = vec![false; self.parameters.num_of_classes as usize];
        self.visited_roomperiods = vec![
            vec![false; self.parameters.num_of_periods as usize];
            self.parameters.num_of_rooms as usize
        ];
        self.corresponding_crp = vec![[0, 0]; self.parameters.num_of_classes as usize];
    }

    pub fn get_corresponding_crp(&self) -> &Vec<[usize; 2]> {
        &self.corresponding_crp
    }

    pub fn get_same_teacher_violations(&self) -> Vec<Violations> {
        let mut res = Vec::new();
        for (_, mp) in (&self.work_periods_each_teachers).iter().enumerate() {
            for (period_id, time) in mp {
                if time.len() > 1 {
                    let violations = Violations::new(*period_id as usize, time.clone());
                    res.push(violations);
                }
            }
        }
        res
    }
    pub fn get_same_students_group_violations(&self) -> Vec<Violations> {
        let mut res = Vec::new();
        for (_, mp) in (&self.work_periods_each_students).iter().enumerate() {
            for (period_id, time) in mp {
                if time.len() > 1 {
                    let violations = Violations::new(*period_id as usize, time.clone());
                    res.push(violations);
                }
            }
        }
        res
    }

    pub fn get_capacity_violations(&self, graph: &Graph) -> Vec<Violations> {
        let mut res = Vec::new();
        for class_id in 0..self.corresponding_crp.len() {
            let [room, period] = self.corresponding_crp[class_id];
            if graph.get_room_ref(room).get_capacity()
                < graph.get_class_ref(class_id).get_num_of_students()
            {
                let mut v = Vec::new();
                v.push(class_id);
                let violations = Violations::new(period as usize, v);
                res.push(violations);
            }
        }
        res
    }

    pub fn get_same_teacher_violations_strictly(&self, input: &Input) -> Vec<Violations> {
        let mut res = Vec::new();
        let period = self.parameters.num_of_periods;
        let room = self.parameters.num_of_rooms;
        let mut table: Vec<Vec<Vec<usize>>> = Vec::with_capacity(period);
        for _ in 0..period {
            let mut period_vec = Vec::with_capacity(room);
            for _ in 0..room {
                period_vec.push(Vec::new());
            }
            table.push(period_vec);
        }
        let classes = input.get_classes();
        for (class_id, [room, period]) in self.corresponding_crp.iter().enumerate() {
            let class_size = classes[class_id].serial_size;
            for i in 0..class_size {
                //ここに教師IDを入れる
                table[period + i][*room].extend(classes[class_id].teacher_indexes.clone());
            }
        }

        let mut same_teacher: Vec<Vec<Vec<usize>>> =
            Vec::with_capacity(self.parameters.num_of_periods);
        for i in 0..self.parameters.num_of_periods {
            same_teacher.push(vec![vec![]; self.parameters.num_of_teachers]);
        }
        for room_id in 0..room {
            for period in 0..period {
                for teacher_id in table[period][room_id].iter() {
                    same_teacher[period][*teacher_id].push(room_id);
                }
            }
        }
        for (i, vv) in same_teacher.iter().enumerate() {
            for v in vv {
                if v.len() > 1 {
                    let mut v = v.clone();
                    v.sort();
                    let violations = Violations::new(i, v);
                    res.push(violations);
                }
            }
        }
        res
    }

    pub fn get_same_students_group_violations_strictly(&self, input: &Input) -> Vec<Violations> {
        let mut res = Vec::new();
        let period = self.parameters.num_of_periods;
        let room = self.parameters.num_of_rooms;
        let mut table: Vec<Vec<Vec<usize>>> = Vec::with_capacity(period);
        for _ in 0..period {
            let mut period_vec = Vec::with_capacity(room);
            for _ in 0..room {
                period_vec.push(Vec::new());
            }
            table.push(period_vec);
        }
        let classes: &Vec<Class> = input.get_classes();
        for (class_id, [room, period]) in self.corresponding_crp.iter().enumerate() {
            let class_size = classes[class_id].serial_size;
            for i in 0..class_size {
                //ここに学生IDをいれる
                table[period + i][*room].extend(classes[class_id].students_group_indexes.clone());
            }
        }
        let mut same_group: Vec<Vec<Vec<usize>>> =
            Vec::with_capacity(self.parameters.num_of_periods);
        for i in 0..self.parameters.num_of_periods {
            same_group.push(vec![vec![]; self.parameters.num_of_students]);
        }
        for room_id in 0..room {
            for period in 0..period {
                for student_id in table[period][room_id].iter() {
                    same_group[period][*student_id].push(room_id);
                }
            }
        }
        for (i, vv) in same_group.iter().enumerate() {
            for v in vv {
                if v.len() > 1 {
                    let mut v = v.clone();
                    v.sort();
                    let violations = Violations::new(i, v);
                    res.push(violations);
                }
            }
        }
        res
    }

    pub fn get_strabble_days_violations(&self, input: &Input) -> Vec<Violations> {
        let mut res = Vec::new();
        let period = self.parameters.num_of_periods;
        for (class_id, [room_id, _]) in self.corresponding_crp.iter().enumerate() {
            let size = input.get_classes()[class_id].serial_size;
            let mut period = period % self.parameters.num_of_day_lengths;
            period += size;
            if period > self.parameters.num_of_day_lengths {
                let mut v = Vec::new();
                v.push(room_id.clone());
                let violations = Violations::new(period, v);
                res.push(violations);
            }
        }
        res
    }
}
