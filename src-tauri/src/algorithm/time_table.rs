//変換を作る
pub mod cell;

use cell::ActiveCell;
use cell::BlankCell;
use rand::seq::index;
use core::str;
use core::time;
use std::error::Error;
use std::sync::Mutex;
use crate::input::class::Class;
use crate::input::room;

use super::aco::aco_solver::ACOSolver;
use super::aco::aco_solver::ACOSolverManager;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimeTable {
    pub class_list: Vec<Option<ActiveCell>>,
    pub process_table: Vec<Vec<Option<Class>>>,
    pub room_size: usize,
    pub period_size: usize,
}
//TODO timeTableに関する操作を抽象化して、それぞれの操作を関数で行う。
//座標とindexを連動させるべきではない

impl TimeTable {
    pub fn new(room_size:usize,period_size:usize,class_size:usize) -> TimeTable{
        let mut class_list = Vec::<Option<ActiveCell>>::new();
        let mut process_table = Vec::<Vec<Option<Class>>>::new();
        for i in 0..room_size {
            let mut row = Vec::<BlankCell>::new();
            for j in 0..period_size {
                row.push(
                    BlankCell{
                        id: i * period_size + j,
                        room: i,
                        period: j,
                        is_visible: true,
                        size: Some(1),
                    }
                );  
            }
        }
        for _ in 0..class_size{
            class_list.push(None);
        }
        for _ in 0..room_size {
            let mut row = Vec::<Option<Class>>::new();
            for _ in 0..period_size {
                row.push(None);
            }
            process_table.push(row);
        }
        TimeTable {
            class_list,
            process_table,
            room_size,
            period_size,
        }
    }

    pub fn get_class(&self,room:usize,period:usize) -> Option<Class>{
        self.process_table[room][period].clone()
    }

    pub fn add_class(&mut self,room:usize,period:usize,class:Class,color:Option<String>){
        for i in 0..class.serial_size {
            self.process_table[room][period+i] = Some(class.clone());
        }
        let id = room * self.period_size + period;
        self.class_list[class.index] = Some(ActiveCell {
            id: id + self.period_size * self.room_size,
            period: period,
            room: room,
            class_index: class.index,
            class_name: format!("{},{},{}",id,class.index,class.name),
            teachers: None,
            students: None,
            color: color,
            is_locked: None,
            size: Some(class.serial_size),
            violations: None,
            tool_tip_message: "".to_string(),
        });
            
    }
    pub fn debug_class_list(&self){
        println!("class_list size{}",self.class_list.len());
        for i in self.class_list.iter() {
            if let Some(class) = i {
                println!("class:{},{}",class.class_index,class.class_name);
            }else{
                println!("None");
            }
        }
    }

    pub fn debug_process_table(&self){
        for i in 0..self.room_size {
            for j in 0..self.period_size {
                if let Some(class) = self.process_table[i][j].as_ref() {
                    print!("{:5}",class.index);
                }else{
                    print!(" None");
                }
            }
            println!("");
        }
    }

    pub fn remove_class(& mut self,room:usize,period:usize){
        println!("remove class:{},{}",room,period);
        let serial_size = self.process_table[room][period].as_ref().unwrap().serial_size;
        let class_index = self.process_table[room][period].as_ref().unwrap().index; 
        for i in 0..serial_size {
            self.process_table[room][period+i] = None;
        }
        self.class_list[class_index] = None;
    }

    pub fn move_class(&mut self,from_room:usize,from_period:usize,to_room:usize,to_period:usize,color:Option<String>){
        let class = self.get_class(from_room, from_period);
        self.remove_class(from_room, from_period);
        self.add_class(to_room, to_period, class.unwrap(),color);
        self.debug_process_table();
    }

}


pub fn convert_solver_to_timetable(solver: &ACOSolver) -> Result<TimeTable, Box<dyn Error>> {
    let mut time_table = TimeTable::new(solver.parameters.num_of_rooms, solver.parameters.num_of_periods,solver.input.get_classes().len());
    let best_ant = solver.get_best_ant().ok_or("No best ant found")?;
    let classes = solver.input.get_classes();
    for (class_id, &[room_id, period_id]) in best_ant.get_corresponding_crp().iter().enumerate() {
        let class = classes[class_id].clone();
        time_table.add_class(room_id, period_id, class, Some(calc_color_init(solver, class_id, room_id, period_id)));
    }
    Ok(time_table)
}

pub struct TimeTableManager {
    pub timetable_manager: Mutex<Option<TimeTable>>,
}

pub fn save_timetable(timetable_manager: tauri::State<'_, TimeTableManager>, timetable: TimeTable) {
    let mut managed_timetable = timetable_manager.timetable_manager.lock().unwrap();
    *managed_timetable = Some(timetable);
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
        let color = (255.0 - (prov * 255.0)) as u8;
        let hex = format!("{:02x}", color);
        res = format!("#ff{}{}ff", hex, hex);
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
    println!("called is_swappable,{},{}", over_id, active_id);
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
    if let Some(time_table) = time_table_manager.timetable_manager.lock().unwrap().as_ref() {
        time_table.debug_process_table();
        time_table.debug_class_list();
        println!("active_id:{},room:{},period:{}",active_id-periods_size*rooms_size,(active_id-rooms_size*periods_size)/periods_size,active_id%periods_size);
        let active_size = time_table.process_table[(active_id-rooms_size*periods_size)/periods_size][active_id%periods_size].as_ref().unwrap().serial_size;
        //no over the day
        let active_index = time_table.process_table[(active_id-rooms_size*periods_size)/periods_size][active_id%periods_size].as_ref().unwrap().index;
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
    println!("called handle_swap_cell,{},{}", over_id, active_id);
    let mut managed_timetable = timetable_manager.timetable_manager.lock().unwrap();
    let solver = solver_manager.solver.lock().unwrap();
    if let Some(time_table) = managed_timetable.as_mut() {
        let room_size = time_table.room_size; 
        let period_size = time_table.period_size;
        let new_id = active_id - period_size*room_size;
        let active_period = new_id % period_size;
        let active_room = new_id / period_size;
        let over_period = over_id % period_size;
        let over_room = over_id / period_size;
        let index = time_table.process_table[active_room][active_period].as_ref().unwrap().index;
        let color = calc_color_from_cell(solver.as_ref().unwrap(), time_table.class_list[index].as_ref().unwrap());
        time_table.move_class(active_room,active_period, over_room, over_period,Some(color));
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
    println!("called handle_switch_lock,{}", id);
    let mut managed_timetable = timetable_manager.timetable_manager.lock().unwrap();
    let solver = solver_manager.solver.lock().unwrap();
    if let Some(time_table) = managed_timetable.as_mut() {
        let room = (id-time_table.room_size * time_table.period_size) / time_table.period_size;
        let period = (id - time_table.room_size * time_table.period_size) % time_table.period_size;
        let class_index = time_table.process_table[room][period].as_ref().unwrap().index;
        
        time_table.class_list[class_index].as_mut().unwrap().is_locked = Some(
            !time_table.class_list[class_index].as_ref().unwrap().is_locked.unwrap_or(false),
        );
        time_table.class_list[class_index].as_mut().unwrap().color = Some(
            calc_color_from_cell(solver.as_ref().unwrap(), time_table.class_list[class_index].as_ref().unwrap()),
        );
        return Ok(time_table.clone());
    }
    return Err("No timetable found".to_string());
}
