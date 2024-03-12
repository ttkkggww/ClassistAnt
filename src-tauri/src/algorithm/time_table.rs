mod cell;

use cell::ActiveCell;
use cell::BlankCell;
use cell::Cell;
use std::error::Error;
use std::sync::Mutex;

use crate::input::room;

use super::aco::aco_solver::ACOSolver;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeTable {
    pub cells: Vec<Cell>,
}

pub fn convert_solver_to_timetable(solver: &ACOSolver) -> Result<TimeTable, Box<dyn Error>> {
    let mut cells = Vec::<Cell>::new();
    let best_ant = solver.get_best_ant().ok_or("No best ant found")?;
    for room in 0..solver.parameters.num_of_rooms {
        for period in 0..solver.parameters.num_of_periods {
            cells.push(Cell::BlankCell(BlankCell {
                id: (room * solver.parameters.num_of_periods + period) as usize,
                size: None,
            }));
        }
    }
    let classes = solver.input.get_classes().clone();
    for (class_id, &[room_id, period_id]) in best_ant.get_corresponding_crp().iter().enumerate() {
        let id = room_id * (solver.parameters.num_of_periods as usize) + period_id;
        cells[room_id as usize * solver.parameters.num_of_periods as usize + period_id as usize] =
            Cell::ActiveCell(ActiveCell {
                id: id as usize,
                class_name: classes[class_id].get_name().clone(),
                teachers: None,
                students: None,
                color: Some(calc_pheromone_color(solver, class_id, room_id, period_id)),
                is_locked: None,
                size: None,
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

fn calc_pheromone_color(
    solver: &ACOSolver,
    class_id: usize,
    room_id: usize,
    period_id: usize,
) -> String {
    let mut res = String::from("FFFFFF");
    if let Some(ant) = solver.get_best_ant() {
        let (rp_v, prov_v) =
            ant.calc_prob_from_v_igunore_visited(class_id, solver.colony.get_graph());
        let period_size = solver.parameters.num_of_rooms as usize;

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
    return res;
}

