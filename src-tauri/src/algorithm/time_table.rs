mod cell;

use cell::Cell;
use std::error::Error;

use super::aco::aco_solver::ACOSolver;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeTable {
    pub cells: Vec<Option<Cell>>,
}

pub fn convert_solver_to_timetable(solver: &ACOSolver) -> Result<TimeTable, Box<dyn Error>> {
    let mut cells = vec![];
    let best_ant = solver.get_best_ant().ok_or("No best ant found")?;
    for room in 0..solver.parameters.num_of_rooms {
        for  period  in 0..solver.parameters.num_of_periods {
            cells.push(None);
        }
    }
    let classes= solver.input.get_classes().clone();
    for (class_id, &[room_id, period_id]) in best_ant.get_corresponding_crp().iter().enumerate(){
        cells[room_id as usize * solver.parameters.num_of_periods as usize + period_id as usize] = Some(Cell{
            class_name : classes[class_id].get_name().clone(),
            teachers:None,
            students:None,
            color:None,
            is_locked:None,
            size:None
        });
    }
    Ok(TimeTable {
        cells,
    })
}