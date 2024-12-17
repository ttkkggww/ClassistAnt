//変換を作る
pub mod cell;

use crate::input::class;
use crate::input::class::Class;
use cell::ActiveCell;
use cell::BlankCell;
use core::str;
use std::error::Error;
use std::os::unix::raw::time_t;
use std::result::Result;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use super::aco::aco_solver::ACOSolver;
use super::aco::aco_solver::ACOSolverManager;
use super::aco::violations::CellsViolation;
use super::aco::violations::Violations;
use crate::input::room::Room;
use serde::{Deserialize, Serialize};
use log::info;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimeTable {
    pub class_list: Vec<Option<ActiveCell>>,
    pub dragging_cell_data: Vec<Vec<Vec<Option<BlankCell>>>>,
    pub process_table: Vec<Vec<Option<Class>>>,
    pub room_size: usize,
    pub period_size: usize,
}
//TODO timeTableに関する操作を抽象化して、それぞれの操作を関数で行う。
//座標とindexを連動させるべきではない

impl TimeTable {
    pub fn new(room_size: usize, period_size: usize, class_size: usize) -> TimeTable {
        let mut class_list = Vec::<Option<ActiveCell>>::new();
        let mut process_table = Vec::<Vec<Option<Class>>>::new();
        for _ in 0..class_size {
            class_list.push(None);
        }
        for _ in 0..room_size {
            let mut row = Vec::<Option<Class>>::new();
            for _ in 0..period_size {
                row.push(None);
            }
            process_table.push(row);
        }
        let dragging_cell_data = vec![vec![vec![None; period_size]; room_size]; class_size];
        TimeTable {
            class_list,
            dragging_cell_data,
            process_table,
            room_size,
            period_size,
        }
    }

    pub fn get_class(&self, room: usize, period: usize) -> Option<Class> {
        self.process_table[room][period].clone()
    }

    pub fn add_class(
        &mut self,
        room: usize,
        period: usize,
        class: Class,
        color: Option<String>,
        solver: &ACOSolver,
    ) {
        for i in 0..class.serial_size {
            self.process_table[room][period + i] = Some(class.clone());
        }
        let mut is_locked = None;
        if let Some(_) = solver.colony.get_graph().get_classes_is_locked(class.index) {
            is_locked = Some(true);
        }
        let id = room * self.period_size + period;
        let tearchers = solver.input.get_teachers();
        self.class_list[class.index] = Some(ActiveCell {
            id: id + self.period_size * self.room_size,
            period: period,
            room: room,
            class_index: class.index,
            class_name: format!("{}", class.name),
            teachers: Some(
                class
                    .teacher_indexes
                    .iter()
                    .map(|&x| tearchers[x].name.clone())
                    .collect(),
            ),
            students: Some(
                class  
                    .students_group_indexes
                    .iter()
                    .map(|&x| solver.input.get_student_groups()[x].name.clone())
                    .collect(),
            ),
            student_size: Some(class.num_of_students),
            color: color,
            is_locked: is_locked,
            size: Some(class.serial_size),
            violations: None,
            tool_tip_message: "".to_string(),
            is_worst_3: None,
        });
    }

    pub fn debug_class_list(&self) {
        println!("class_list size{}", self.class_list.len());
        for i in self.class_list.iter() {
            if let Some(class) = i {
                println!("class:{},{}", class.class_index, class.class_name);
            } else {
                println!("None");
            }
        }
    }

    pub fn debug_process_table(&self) {
        for i in 0..self.room_size {
            for j in 0..self.period_size {
                if let Some(class) = self.process_table[i][j].as_ref() {
                    print!("{:5}", class.index);
                } else {
                    print!(" None");
                }
            }
            println!("");
        }
    }

    fn update_violations(
        &mut self,
        room: usize,
        period: usize,
        room_list: &Vec<Room>,
        one_day_length: usize,
    ) {
        let violations = self.get_new_violations(room, period, room_list, one_day_length);
        let class_idx = self.process_table[room][period].as_ref().unwrap().index;
        self.class_list[class_idx].as_mut().unwrap().violations = Some(violations);
    }

    pub fn remove_class(&mut self, room: usize, period: usize) {
        let serial_size = self.process_table[room][period]
            .as_ref()
            .unwrap()
            .serial_size;
        let class_index = self.process_table[room][period].as_ref().unwrap().index;
        for i in 0..serial_size {
            self.process_table[room][period + i] = None;
        }
        self.class_list[class_index] = None;
    }

    pub fn move_class(
        &mut self,
        from_room: usize,
        from_period: usize,
        to_room: usize,
        to_period: usize,
        color: Option<String>,
        solver: &ACOSolver,
    ) {
        let class = self.get_class(from_room, from_period);
        let mut pre_violations: Option<CellsViolation> = None;
        if let Some(input_class) = &class {
            if let Some(cell_input) = &self.class_list[input_class.index] {
                pre_violations = cell_input.violations.clone();
            }
        }
        self.remove_class(from_room, from_period);
        self.add_class(to_room, to_period, class.clone().unwrap(), color, solver);
        let room_list = solver.input.get_rooms();
        self.lock(class.as_ref().unwrap().index).unwrap();
        
        let violations = self.get_new_violations(
            to_room,
            to_period,
            room_list,
            solver.parameters.num_of_day_lengths,
        );
        let class_idx = self.process_table[to_room][to_period]
            .as_ref()
            .unwrap()
            .index;
        self.class_list[class_idx].as_mut().unwrap().violations = Some(violations);
        if let Some(violations) = pre_violations {
            for violation in violations.same_student_same_time {
                let period = violation.period;
                for room in violation.rooms {
                    self.update_violations(
                        room,
                        period,
                        room_list,
                        solver.parameters.num_of_day_lengths,
                    );
                }
            }
        }
        let mut post_violations: Option<CellsViolation> = None;
        if let Some(input_class) = &class {
            if let Some(cell_input) = &self.class_list[input_class.index] {
                post_violations = cell_input.violations.clone();
            }
        }
        if let Some(violations) = post_violations {
            for violation in violations.same_student_same_time {
                let period = violation.period;
                for room in violation.rooms {
                    self.update_violations(
                        room,
                        period,
                        room_list,
                        solver.parameters.num_of_day_lengths,
                    );
                }
            }
        }
    }

    pub fn calc_same_student_same_time(&self, room_id: usize, period_id: usize) -> Vec<Violations> {
        let mut violations = Vec::<Violations>::new();
        let serial_size = self.process_table[room_id][period_id]
            .as_ref()
            .unwrap()
            .serial_size;
        let students = self.process_table[room_id][period_id]
            .as_ref()
            .unwrap()
            .students_group_indexes
            .clone();
        let class_idx = self.process_table[room_id][period_id]
            .as_ref()
            .unwrap()
            .index;
        for time in period_id..(period_id + serial_size) {
            for room in 0..self.room_size {
                if time == period_id && room == room_id {
                    continue;
                }
                if let Some(class) = self.process_table[room][time].as_ref() {
                    if class.index == class_idx {
                        continue;
                    }
                    let mut common_students = class.students_group_indexes.clone();
                    common_students.retain(|&x| students.contains(&x));
                    if common_students.len() > 0 {
                        violations.push(Violations {
                            period: time,
                            rooms: vec![room],
                        });
                    }
                }
            }
        }
        violations
    }

    pub fn common_ids(ids1: &Vec<usize>, ids2: &Vec<usize>) -> Vec<usize> {
        let mut common_ids = Vec::<usize>::new();
        for id in ids1 {
            if ids2.contains(id) {
                common_ids.push(*id);
            }
        }
        common_ids
    }

    pub fn calc_same_teacher_same_time(&self, room_id: usize, period_id: usize) -> Vec<Violations> {
        let mut violations = Vec::<Violations>::new();
        let serial_size = self.process_table[room_id][period_id]
            .as_ref()
            .unwrap()
            .serial_size;
        let teachers = self.process_table[room_id][period_id]
            .as_ref()
            .unwrap()
            .teacher_indexes
            .clone();
        let class_idx = self.process_table[room_id][period_id]
            .as_ref()
            .unwrap()
            .index;
        for time in period_id..(period_id + serial_size) {
            for room in 0..self.room_size {
                if time == period_id && room == room_id {
                    continue;
                }
                if let Some(class) = self.process_table[room][time].as_ref() {
                    if class.index == class_idx {
                        continue;
                    }
                    let mut common_teachers = class.teacher_indexes.clone();
                    common_teachers.retain(|&x| teachers.contains(&x));
                    if common_teachers.len() > 0 {
                        violations.push(Violations {
                            period: time,
                            rooms: vec![room],
                        });
                    }
                }
            }
        }
        violations
    }

    pub fn calc_capacity_over(
        &self,
        room_id: usize,
        period_id: usize,
        room_list: &Vec<Room>,
    ) -> Vec<Violations> {
        let mut violations = Vec::<Violations>::new();
        let num_of_students = self.process_table[room_id][period_id]
            .as_ref()
            .unwrap()
            .num_of_students;
        let capacity = room_list[room_id].capacity;
        if num_of_students > capacity {
            violations.push(Violations {
                period: period_id,
                rooms: vec![room_id],
            });
        }
        violations
    }

    pub fn calc_strabble_days(
        &self,
        room_id: usize,
        period_id: usize,
        one_day_length: usize,
    ) -> Vec<Violations> {
        let mut violations = Vec::<Violations>::new();
        let serial_size = self.process_table[room_id][period_id]
            .as_ref()
            .unwrap()
            .serial_size;
        let start_in_a_day = period_id % one_day_length;
        if start_in_a_day + serial_size > one_day_length {
            violations.push(Violations {
                period: period_id,
                rooms: vec![room_id],
            });
        }
        violations
    }

    pub fn get_new_violations(
        &self,
        room_id: usize,
        period_id: usize,
        room_list: &Vec<Room>,
        one_day_length: usize,
    ) -> CellsViolation {
        let same_student_same_time = self.calc_same_student_same_time(room_id, period_id);
        let same_teacher_same_time = self.calc_same_teacher_same_time(room_id, period_id);
        let capacity_over = self.calc_capacity_over(room_id, period_id, room_list);
        let strabble_days = self.calc_strabble_days(room_id, period_id, one_day_length);
        let mut is_violated: bool = false;
        if same_student_same_time.len() > 0
            || same_teacher_same_time.len() > 0
            || capacity_over.len() > 0
            || strabble_days.len() > 0
        {
            is_violated = true;
        }
        CellsViolation {
            is_violated,
            same_student_same_time,
            same_teacher_same_time,
            capacity_over,
            strabble_days,
        }
    }

    pub fn updated_by_process_table(&mut self, solver: &ACOSolver) {
        let is_locked_list = self
            .class_list
            .iter()
            .map(|x| x.as_ref().unwrap().is_locked.unwrap_or(false))
            .collect::<Vec<bool>>();
        self.class_list = Vec::<Option<ActiveCell>>::new();
        self.dragging_cell_data = vec![
            vec![vec![None; self.period_size]; self.room_size];
            solver.input.get_classes().len()
        ];
        let teachers = solver.input.get_teachers();
        for i in 0..self.room_size {
            for j in 0..self.period_size {
                if let Some(class) = self.process_table[i][j].as_ref() {
                    let id = i * self.period_size + j;
                    self.class_list[class.index] = Some(ActiveCell {
                        id: id + self.period_size * self.room_size,
                        period: j,
                        room: i,
                        class_index: class.index,
                        class_name: format!("{},{},{}", id, class.index, class.name),
                        teachers: Some(
                            class
                                .teacher_indexes
                                .iter()
                                .map(|&x| teachers[x].name.clone())
                                .collect(),
                        ),
                        students: None,
                        student_size: Some(class.num_of_students),
                        color: None, //ここsolverから取得する
                        is_locked: Some(is_locked_list[class.index]),
                        size: Some(class.serial_size),
                        violations: None,
                        tool_tip_message: "".to_string(),
                        is_worst_3: None,
                    });
                } else {
                    self.class_list.push(None);
                }
            }
        }
    }
    
    pub fn update_worst3_cell(&mut self, solver: &ACOSolver) {
        //calc_prob_from_v_igunore_visitedが下から3番目までのclass_listのworst3をtrueにする
        let mut prob_tuple = Vec::<(f64, usize)>::new();
        for i in 0..self.class_list.len() {
            if let Some(cell) = &self.class_list[i] {
                if let Some(ant) = solver.get_best_ant() {
                    let (rp_v, prov_v) =
                        ant.calc_prob_from_v_igunore_visited(cell.class_index, solver.colony.get_graph());
                    let mut prov = 0.0;
                    for (i, rp) in rp_v.iter().enumerate() {
                        if rp[0] == cell.room && rp[1] == cell.period {
                            prov = prov_v[i];
                        }
                    }
                    prob_tuple.push((prov, i));
                }
            }
        }
        prob_tuple.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        for i in self.class_list.iter_mut() {
            if let Some(cell) = i {
                cell.is_worst_3 = Some(false);
            }
        }
        //3番目までをtrueにする
        for i in 0..3 {
            if let Some(cell) = &mut self.class_list[prob_tuple[i].1] {
                cell.is_worst_3 = Some(true);
            }
        }
    }

    pub fn lock(&mut self, class_index: usize) -> Result<(), String> {
        self.class_list[class_index].as_mut().unwrap().is_locked = Some(true);
        self.class_list[class_index].as_mut().unwrap().color = Some(
            "#AAAAFF".to_string(),
        );
        return Ok(());
    }
    pub fn unlock(&mut self, class_index: usize,solver: &ACOSolver) -> Result<(), String> {
        self.class_list[class_index].as_mut().unwrap().is_locked = Some(false);
        self.class_list[class_index].as_mut().unwrap().color = Some(
            calc_color_from_cell(
                &solver,
                self.class_list[class_index].as_ref().unwrap(),
            ),
        );
        return Ok(());
    }
}

pub fn convert_solver_to_timetable(solver: &ACOSolver) -> Result<TimeTable, Box<dyn Error>> {
    let mut time_table = TimeTable::new(
        solver.parameters.num_of_rooms,
        solver.parameters.num_of_periods,
        solver.input.get_classes().len(),
    );
    let best_ant = solver.get_best_ant().ok_or("No best ant found")?;
    let classes = solver.input.get_classes();
    for (class_id, &[room_id, period_id]) in best_ant.get_corresponding_crp().iter().enumerate() {
        let class = classes[class_id].clone();
        time_table.add_class(
            room_id,
            period_id,
            class,
            Some(calc_color_init(solver, class_id, room_id, period_id)),
            solver,
        );
    }
    for cell in time_table.clone().class_list {
        if let Some(cell) = cell {
            let violations = Some(time_table.get_new_violations(
                cell.room,
                cell.period,
                &solver.input.get_rooms(),
                solver.parameters.num_of_day_lengths,
            ));
            //println!("violations:{:?}",&violations);
            time_table.class_list[cell.class_index]
                .as_mut()
                .unwrap()
                .violations = violations;
        }
    }
    time_table.update_worst3_cell(solver);
    Ok(time_table)
}

pub struct TimeTableManager {
    pub timetable_manager: Mutex<Option<TimeTable>>,
}

pub fn save_timetable(timetable_manager: tauri::State<'_, TimeTableManager>, timetable: TimeTable) {
    let mut managed_timetable = timetable_manager.timetable_manager.lock().unwrap();
    *managed_timetable = Some(timetable);
}

pub fn save_solver(
    solver_manager: tauri::State<'_, ACOSolverManager>,
    solver: ACOSolver,
) -> Result<(), Box<dyn Error>> {
    let mut managed_solver = solver_manager.solver.lock().unwrap();
    *managed_solver = Some(solver);
    Ok(())
}

fn calc_color_init(
    solver: &ACOSolver,
    class_id: usize,
    room_id: usize,
    period_id: usize,
) -> String {
    let mut res = get_pheromone_color(solver, class_id, room_id, period_id);

    if let Some(is_lock) = solver.colony.get_graph().get_classes_is_locked(class_id) {
        if is_lock.0 == room_id as usize && is_lock.1 == period_id as usize {
            res = "#AAAAFF".to_string();
        }
    }
    return res;
}

fn calculate_gradient_color(prov: f64, start_color: &str, end_color: &str) -> String {
    // パースした色の値を取得 (R, G, B)
    let start_r = u8::from_str_radix(&start_color[1..3], 16).unwrap();
    let start_g = u8::from_str_radix(&start_color[3..5], 16).unwrap();
    let start_b = u8::from_str_radix(&start_color[5..7], 16).unwrap();

    let end_r = u8::from_str_radix(&end_color[1..3], 16).unwrap();
    let end_g = u8::from_str_radix(&end_color[3..5], 16).unwrap();
    let end_b = u8::from_str_radix(&end_color[5..7], 16).unwrap();

    // 線形補間を使用して各色成分を計算
    let r = ((1.0 - prov) * start_r as f64 + prov * end_r as f64) as u8;
    let g = ((1.0 - prov) * start_g as f64 + prov * end_g as f64) as u8;
    let b = ((1.0 - prov) * start_b as f64 + prov * end_b as f64) as u8;

    // 計算したRGB値から16進数文字列を生成
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

fn get_pheromone_color(
    solver: &ACOSolver,
    class_id: usize,
    room_id: usize,
    period_id: usize,
) -> String {
    let mut res = String::from("#FFFFFF");
    if let Some(ant) = solver.get_best_ant() {
        let (rp_v, prov_v) =
            ant.calc_prob_from_v_igunore_visited(class_id, solver.colony.get_graph());
        let mut prov = 0.0;
        for (i, rp) in rp_v.iter().enumerate() {
            if rp[0] == room_id && rp[1] == period_id {
                prov = prov_v[i];
            }
        }
        res = calculate_gradient_color(prov, "#FFFFFF", "#5CB85C");
    }
    res
}

fn calc_color_from_cell(solver: &ACOSolver, active_cell: &ActiveCell) -> String {
    if active_cell.is_locked.unwrap_or(false) {
        return "#AAAAFF".to_string();
    }
    let class_id = active_cell.class_index;
    let room_id = active_cell.room;
    let period_id = active_cell.period;
    return get_pheromone_color(solver, class_id, room_id, period_id);
}

#[tauri::command]
pub fn is_swappable(
    time_table_manager: tauri::State<'_, TimeTableManager>,
    solver_manager: tauri::State<'_, ACOSolverManager>,
    over_id: usize,
    active_id: usize,
) -> Result<bool, String> {
    info!("called is_swappable,{},{}", over_id, active_id);
    let managed_solver = solver_manager.solver.lock().unwrap();
    let parameter = managed_solver
        .as_ref()
        .ok_or("No solver found")?
        .parameters
        .clone();

    let periods_size = parameter.num_of_periods;
    let rooms_size = parameter.num_of_rooms;
    let mut is_swappable = true;
    let over_room = over_id / periods_size;
    let over_period = over_id % periods_size;
    if let Some(time_table) = time_table_manager
        .timetable_manager
        .lock()
        .unwrap()
        .as_ref()
    {
        info!(
            "active_id:{},room:{},period:{}",
            active_id - periods_size * rooms_size,
            (active_id - rooms_size * periods_size) / periods_size,
            active_id % periods_size
        );
        let active_size = time_table.process_table
            [(active_id - rooms_size * periods_size) / periods_size][active_id % periods_size]
            .as_ref()
            .unwrap()
            .serial_size;
        //no over the day
        let active_index = time_table.process_table
            [(active_id - rooms_size * periods_size) / periods_size][active_id % periods_size]
            .as_ref()
            .unwrap()
            .index;
        for day in over_period..(over_period + active_size) {
            if day >= periods_size {
                is_swappable = false;
                break;
            }
            if let Some(class) = time_table.process_table[over_room][day].as_ref() {
                if class.index != active_index {
                    is_swappable = false;
                    break;
                }
            }
        }
    }
    return Ok(is_swappable);
}

//Assume all swap destinations are blankcells
#[tauri::command]
pub fn handle_swap_cell(
    timetable_manager: tauri::State<'_, TimeTableManager>,
    solver_manager: tauri::State<'_, ACOSolverManager>,
    over_id: usize,
    active_id: usize,
) -> Result<TimeTable, String> {
    info!("called handle_swap_cell,{},{}", over_id, active_id);
    let mut managed_timetable = timetable_manager.timetable_manager.lock().unwrap();
    let solver = solver_manager.solver.lock().unwrap();
    if let Some(time_table) = managed_timetable.as_mut() {
        let room_size = time_table.room_size;
        let period_size = time_table.period_size;
        let new_id = active_id - period_size * room_size;
        let active_period = new_id % period_size;
        let active_room = new_id / period_size;
        let over_period = over_id % period_size;
        let over_room = over_id / period_size;
        let index = time_table.process_table[active_room][active_period]
            .as_ref()
            .unwrap()
            .index;
        let mut color =
            get_pheromone_color(solver.as_ref().unwrap(), index, over_room, over_period);
        let is_locked = time_table.class_list[index]
            .as_ref()
            .unwrap()
            .is_locked
            .unwrap_or(false);
        if is_locked {
            color = "#AAAAFF".to_string();
        }
        time_table.move_class(
            active_room,
            active_period,
            over_room,
            over_period,
            Some(color),
            solver.as_ref().unwrap(),
        );
        time_table.update_worst3_cell(solver.as_ref().unwrap());
        return Ok(time_table.clone());
    }
    return Err("No timetable found".to_string());
}

#[tauri::command]
pub fn handle_switch_lock(
    timetable_manager: tauri::State<'_, TimeTableManager>,
    solver_manager: tauri::State<'_, ACOSolverManager>,
    id: usize,
) -> Result<TimeTable, String> {
    info!("called handle_switch_lock,{}", id);
    let mut managed_timetable = timetable_manager.timetable_manager.lock().unwrap();
    let solver = solver_manager.solver.lock().unwrap();
    if let Some(time_table) = managed_timetable.as_mut() {
        let room = (id - time_table.room_size * time_table.period_size) / time_table.period_size;
        let period = (id - time_table.room_size * time_table.period_size) % time_table.period_size;
        let class_index = time_table.process_table[room][period]
            .as_ref()
            .unwrap()
            .index;

        time_table.class_list[class_index]
            .as_mut()
            .unwrap()
            .is_locked = Some(
            !time_table.class_list[class_index]
                .as_ref()
                .unwrap()
                .is_locked
                .unwrap_or(false),
        );
        time_table.class_list[class_index].as_mut().unwrap().color = Some(calc_color_from_cell(
            solver.as_ref().unwrap(),
            time_table.class_list[class_index].as_ref().unwrap(),
        ));
        return Ok(time_table.clone());
    }
    return Err("No timetable found".to_string());
}


#[tauri::command]
pub fn handle_lock_no_violation(
    timetable_manager: tauri::State<'_, TimeTableManager>,
) -> Result<TimeTable, String> {
    info!("called handle_rock_allof_violation");
    let mut managed_timetable = timetable_manager.timetable_manager.lock().unwrap();
    if let Some(time_table) = managed_timetable.as_mut() {
        for i in 0..time_table.class_list.len() {
            if let Some(cell) = &time_table.class_list[i] {
                if !cell.violations.as_ref().unwrap().is_violated {
                    time_table.lock(i).unwrap();
                }
            }
        }
        return Ok(time_table.clone());
    }
    return Err("No timetable found".to_string());
}

#[tauri::command]
pub fn handle_unlock_violation(
    timetable_manager: tauri::State<'_, TimeTableManager>,
    solver_manager: tauri::State<'_, ACOSolverManager>,
) -> Result<TimeTable, String> {
    info!("called handle_unlock_violation");
    let mut managed_timetable = timetable_manager.timetable_manager.lock().unwrap();
    let solver = solver_manager.solver.lock().unwrap();
    if let Some(time_table) = managed_timetable.as_mut() {
        for i in 0..time_table.class_list.len() {
            if let Some(cell) = &time_table.class_list[i] {
                if cell.violations.as_ref().unwrap().is_violated {
                    time_table.unlock(i, solver.as_ref().unwrap()).unwrap();
                }
            }
        }
        return Ok(time_table.clone());
    }
    return Err("No timetable found".to_string());
}


const DUMP_PATH: &str = "ClassistAnt";
const DUMP_TIMETABLE_FILE: &str = "timetable.json";
const DUMP_SOLVER_FILE: &str = "solver.json";
use tauri::api::path::config_dir;
#[tauri::command]
pub fn dump_timetable(
    timetable_manager: tauri::State<'_, TimeTableManager>,
    solver_manager: tauri::State<'_, ACOSolverManager>,
) {
    info!("called dump_timetable");
    let json = serde_json::to_string(
        timetable_manager
            .timetable_manager
            .lock()
            .unwrap()
            .as_ref()
            .unwrap(),
    )
    .unwrap();
    if let Some(mut path) = config_dir() {
        path.push(DUMP_PATH);
        std::fs::create_dir_all(&path).unwrap();
        path.push(DUMP_TIMETABLE_FILE);
        let mut file = File::create(path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
    let solver_json =
        serde_json::to_string(solver_manager.solver.lock().unwrap().as_ref().unwrap()).unwrap();
    if let Some(mut path) = config_dir() {
        path.push(DUMP_PATH);
        std::fs::create_dir_all(&path).unwrap();
        path.push(DUMP_SOLVER_FILE);
        let mut file = File::create(path).unwrap();
        file.write_all(solver_json.as_bytes()).unwrap();
    }
}

#[tauri::command]
pub fn load_timetable(
    timetable_manager: tauri::State<'_, TimeTableManager>,
    solver_manager: tauri::State<'_, ACOSolverManager>,
) -> Result<TimeTable, String> {
    info!("called load_timetable");
    if let Some(mut path) = config_dir() {
        path.push(DUMP_PATH);
        path.push(DUMP_SOLVER_FILE);
        let mut file = File::open(path).unwrap();
        let mut json = String::new();
        file.read_to_string(&mut json).unwrap();
        let solver: ACOSolver = serde_json::from_str(&json).unwrap();
        save_solver(solver_manager, solver.clone()).unwrap();
    }
    if let Some(mut path) = config_dir() {
        path.push(DUMP_PATH);
        std::fs::create_dir_all(&path).unwrap();
        path.push(DUMP_TIMETABLE_FILE);
        let mut file = File::open(path).unwrap();
        let mut json = String::new();
        file.read_to_string(&mut json).unwrap();
        let timetable: TimeTable = serde_json::from_str(&json).unwrap();
        save_timetable(timetable_manager, timetable.clone());
        return Ok(timetable);
    }
    return Err("No timetable found".to_string());
}
