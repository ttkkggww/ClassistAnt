pub mod cell;

use cell::ActiveCell;
use cell::BlankCell;
use cell::Cell;
use rand::seq::index;
use core::str;
use std::error::Error;
use std::fmt::format;
use std::sync::Mutex;
use crate::input::class;

use super::aco::aco_solver::ACOSolver;
use super::aco::aco_solver::ACOSolverManager;
use super::aco::violations;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeTable {
    pub cells: Vec<Cell>,
}

pub fn convert_solver_to_timetable(solver: &ACOSolver) -> Result<TimeTable, Box<dyn Error>> {
    let mut cells = Vec::<Cell>::new();
    //TODO:ここの変換の調整
    let best_ant = solver.get_best_ant().ok_or("No best ant found")?;
    for room in 0..solver.parameters.num_of_rooms {
        for period in 0..solver.parameters.num_of_periods {
            cells.push(Cell::BlankCell(BlankCell {
                id: (room * solver.parameters.num_of_periods + period) as usize,
                room: room as usize,
                period: period as usize,
                is_visible: true,
                size: Some(1),
            }));
        }
    }
    let classes = solver.input.get_classes().clone();
    let mut cells_violations = Vec::new();
    let mut tool_tip_message = Vec::new();
    {
        for i in 0..(solver.parameters.num_of_classes) {
            cells_violations.push(
                violations::CellsViolation {
                    is_violated: false,
                    same_student_same_time: Vec::new(),
                    same_teacher_same_time: Vec::new(),
                    capacity_over: Vec::new(),
                    strabble_days: Vec::new(),
                }
            );
            tool_tip_message.push("".to_string());
        }
        let same_tercher_same_time = solver.get_best_ant_same_teacher_violations_strictly();
        let same_student_same_time = solver.get_best_ant_same_group_violations_strictly();
        let capacity_over = solver.get_best_ant_capacity_violations();
        let strabble_days = solver.get_best_ant_strabble_days_violations();
        let class_index_talbe = solver.get_class_index_time_table();
        for violation in same_tercher_same_time {
            for room in violation.clone().rooms {
                let period = violation.period;
                let index = class_index_talbe[room][period];
                cells_violations[index].is_violated = true;
                cells_violations[index].same_teacher_same_time.push(violation.clone());
                let room_names: Vec<_> = violation.rooms.clone()
                    .iter()
                    .map(|room| solver.input.get_rooms()[*room].name.clone())
                    .collect();
                tool_tip_message[index] =  format!("{}\n同じ教師が同じ時間に複数のクラスを持っています。{:?}",tool_tip_message[index],room_names);
            }
            
        }
        for violation in same_student_same_time {
            for room in violation.clone().rooms {
                let period = violation.period;
                let index   = class_index_talbe[room][period];
                cells_violations[index].is_violated = true;
                cells_violations[index].same_student_same_time.push(violation.clone());
                let room_names: Vec<_> = violation.rooms.clone()
                    .iter()
                    .map(|room| solver.input.get_rooms()[*room].name.clone())
                    .collect();
                tool_tip_message[index] = format!("{}\n同じ生徒が同じ時間に複数のクラスを持っています。{:?}",tool_tip_message[index],room_names);
            }
        }
        for violation in capacity_over {
            for room in violation.clone().rooms {
                let period = violation.period;
                cells_violations[room].is_violated = true;
                cells_violations[room].capacity_over.push(violation.clone());
                tool_tip_message[room] = format!("{}\n教室のキャパシティを超えています。",tool_tip_message[room]);
            }
        }
        for strabble_day in strabble_days {
            for room in strabble_day.clone().rooms {
                let period = strabble_day.period;
                let index = class_index_talbe[room][period];
                cells_violations[index].is_violated = true;
                cells_violations[index].strabble_days.push(strabble_day.clone());
                tool_tip_message[index] = format!("{}\n教室が日を跨いでいます。",tool_tip_message[index]);
            }
        }
    }
    for (class_id, &[room_id, period_id]) in best_ant.get_corresponding_crp().iter().enumerate() {
        let id = room_id * (solver.parameters.num_of_periods as usize) + period_id;
        let class = classes[class_id].clone();
        let teacher_names: Vec<_>= class.teacher_indexes.iter().map(|x| solver.input.get_teachers()[*x].name.clone()).collect();
        cells[room_id as usize * solver.parameters.num_of_periods as usize + period_id as usize] =
            Cell::ActiveCell(ActiveCell {
                id: id as usize,
                period: period_id as usize,
                room: room_id as usize,
                class_index: class_id as usize,
                class_name: format!(
                    "{}:{}:{:?}",
                    id.to_string(),
                    (classes[class_id].get_name().clone()),
                    teacher_names
                ),
                teachers: None,
                students: None,
                color: Some(calc_color_init(solver, class_id, room_id, period_id)),
                is_locked: solver
                    .colony
                    .get_graph()
                    .get_classes_is_locked(class_id)
                    .map(|_| true),
                size: Some(classes[class_id].serial_size),
                violations: Some(cells_violations[class_id].clone()),
                tool_tip_message: tool_tip_message[class_id].clone(),
            });
    }

    Ok(TimeTable { cells })
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
    timetable_manager: tauri::State<'_, TimeTableManager>,
    solver_manager: tauri::State<'_, ACOSolverManager>,
    over_id: usize,
    active_id: usize,
) -> Result<bool, String> {
    let managed_timetable = timetable_manager.timetable_manager.lock().unwrap();
    let managed_solver = solver_manager.solver.lock().unwrap();
    let parameter = managed_solver
        .as_ref()
        .ok_or("No solver found")?
        .parameters
        .clone();
    let periods_size = parameter.num_of_periods;
    let mut over_remain = over_id % periods_size;
    let mut is_swappable = true;
    if let Some(timetable) = managed_timetable.as_ref() {
        if let Cell::ActiveCell(active_cell) = timetable.cells[active_id].clone() {
            for index in over_id..(over_id + active_cell.size.unwrap_or(1)) {
                if let Cell::ActiveCell(active_cell) = timetable.cells[index].clone() {
                    if active_cell.id != active_id {
                        is_swappable = false;
                    }
                }
            }
            over_remain += active_cell.size.unwrap_or(1);
        }
    }
    if over_remain > periods_size {
        is_swappable = false;
    }
    return Ok(is_swappable);
}

//Assume all swap destinations are blankcells
#[tauri::command]
pub fn handle_swap_cell(
    timetable_manager: tauri::State<'_, TimeTableManager>,
    over_id: usize,
    active_id: usize,
) -> Result<TimeTable, String> {
    let mut managed_timetable = timetable_manager.timetable_manager.lock().unwrap();
    let mut new_timetable;
    if let Some(timetable) = managed_timetable.as_mut() {
        new_timetable = timetable.clone();
        if let Cell::ActiveCell(active_cell) = new_timetable.cells[active_id].clone() {
            if let Cell::BlankCell(blank_cell) = new_timetable.cells[over_id].clone() {
                new_timetable.cells[active_id] = Cell::BlankCell(blank_cell.clone());
                match &mut new_timetable.cells[active_id] {
                    Cell::BlankCell(blank_cell) => {
                        blank_cell.id = active_cell.id;
                        blank_cell.room = active_cell.room;
                        blank_cell.period = active_cell.period.clone();
                        blank_cell.is_visible = true;
                    }
                    _ => (),
                }
                new_timetable.cells[over_id] = Cell::ActiveCell(active_cell.clone());
                match &mut new_timetable.cells[over_id] {
                    Cell::ActiveCell(active_cell) => {
                        active_cell.class_name =
                            format!("{}:{}", over_id.to_string(), active_cell.class_name.clone());
                        active_cell.id = blank_cell.id;
                        active_cell.room = blank_cell.room;
                        active_cell.period = blank_cell.period;
                    }
                    _ => (),
                }
                println!("Swaped cells");
                match &mut new_timetable.cells[over_id] {
                    Cell::ActiveCell(active_cell) => {
                        active_cell.is_locked = Some(true);
                        active_cell.color = Some("#AAAAFF".to_string());
                    }
                    _ => (),
                }
            }
        }
    } else {
        return Err("No timetable found".to_string());
    }
    let active_cell_size = match &new_timetable.cells[over_id] {
        Cell::ActiveCell(active_cell) => active_cell.size.unwrap_or(1),
        _ => 1,
    };
    *managed_timetable = Some(new_timetable.clone());
    return Ok(new_timetable);
}

#[tauri::command]
pub fn handle_switch_lock(
    timetable_manager: tauri::State<'_, TimeTableManager>,
    acosolver_manager: tauri::State<'_, ACOSolverManager>,
    id: usize,
) -> Result<TimeTable, String> {
    println!("called handle_switch_lock,{}", id);
    //このidはindexを指さない
    let mut managed_timetable = timetable_manager.timetable_manager.lock().unwrap();
    let mut managed_solver = acosolver_manager.solver.lock().unwrap();
    if let Some(timetable) = managed_timetable.as_mut() {
        if let Some(solver) = managed_solver.as_mut() {
            if let Cell::ActiveCell(active_cell) = timetable.cells[id].as_mut() {
                active_cell.is_locked = Some(!active_cell.is_locked.unwrap_or(false));
                active_cell.color = Some(calc_color_from_cell(solver, active_cell));
            }
        }
        return Ok(timetable.clone());
    }
    return Err("No timetable found".to_string());
}
