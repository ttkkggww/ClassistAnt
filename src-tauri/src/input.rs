use serde::{Deserialize, Serialize};

use self::{student_group::StudentGroup, teacher::Teacher};
use std::sync::Mutex;
use std::{error::Error, vec};
use log::info;

pub mod class;
mod column;
pub mod room;
mod student_group;
pub mod teacher;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Input {
    classes: Vec<class::Class>,
    rooms: Vec<room::Room>,
    student_groups: Vec<student_group::StudentGroup>,
    teachers: Vec<teacher::Teacher>,
}

const TEACHERS_CSV_PATH: &str = "./csvdata/teachers.csv";
const STUDENT_GROUPS_CSV_PATH: &str = "./csvdata/student_groups.csv";
const CLASSES_CSV_PATH: &str = "./csvdata/classes.csv";
const ROOMS_CSV_PATH: &str = "./csvdata/rooms.csv";

impl Input {
    pub fn new() -> Input {
        let teachers = Input::read_teachers_from_csv(&TEACHERS_CSV_PATH.to_string()).unwrap();
        let rooms = Input::read_rooms_from_csv(&ROOMS_CSV_PATH.to_string()).unwrap();
        let student_groups =
            Input::read_student_groups_from_csv(&STUDENT_GROUPS_CSV_PATH.to_string()).unwrap();
        let classes = Input::read_classes_from_csv(
            &CLASSES_CSV_PATH.to_string(),
            &teachers,
            &rooms,
            &student_groups,
        )
        .unwrap();
        Input {
            classes,
            rooms,
            student_groups,
            teachers,
        }
    }

    fn read_teachers_from_csv(file_path: &String) -> Result<Vec<teacher::Teacher>, Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path(file_path)?;
        let mut teachers = Vec::new();
        for (index, result) in rdr.records().enumerate() {
            let record = result?;
            let id = record[0].parse::<usize>().unwrap();
            let name = record[1].to_string();
            let absent_days = if record[2].is_empty() {
                vec![]
            } else {
                record[2]
                    .split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect()
            };
            let index = index as usize;
            teachers.push(teacher::Teacher {
                id,
                index,
                name,
                absent_days,
            });
        }

        Ok(teachers)
    }

    fn read_rooms_from_csv(file_path: &String) -> Result<Vec<room::Room>, Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path(file_path)?;
        let mut rooms = Vec::new();
        for (index, result) in rdr.records().enumerate() {
            let record = result?;
            let index = index as usize;
            let id = record[0].parse::<usize>().unwrap();
            let name = record[1].to_string();
            let capacity = record[2].parse::<usize>().unwrap();
            rooms.push(room::Room {
                id,
                index,
                name,
                capacity,
            });
        }
        Ok(rooms)
    }

    fn read_student_groups_from_csv(
        file_path: &String,
    ) -> Result<Vec<student_group::StudentGroup>, Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path(file_path)?;
        let mut student_groups = Vec::new();
        for (index, result) in rdr.records().enumerate() {
            let record = result?;
            let id = record[0].parse::<usize>().unwrap();
            let name = record[1].to_string();
            let index = index as usize;
            student_groups.push(student_group::StudentGroup { id, index, name });
        }
        Ok(student_groups)
    }

    fn read_classes_from_csv(
        file_path: &String,
        teachers: &Vec<Teacher>,
        rooms: &Vec<room::Room>,
        student_groups: &Vec<StudentGroup>,
    ) -> Result<Vec<class::Class>, Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path(file_path)?;
        let mut classes = Vec::new();

        for (index, result) in rdr.records().enumerate() {
            let record = result?;
            let index = index as usize;
            let id = record[0].parse::<usize>().unwrap();
            let name = record[1].to_string();
            let mut teacher_indexes = Vec::new();
            for i in record[2].split(",") {
                if i == "" {
                    continue;
                }
                if let Some(add) = teachers.iter().position(|x| x.name == i) {
                    teacher_indexes.push(add as usize);
                } else {
                    panic!("teacher not found {}", i);
                }
            }
            let mut room_candidates_indexes = Vec::new();
            for i in record[3].split(",") {
                if let Some(add) = rooms.iter().position(|x| x.name == i) {
                    room_candidates_indexes.push(add as usize);
                } else {
                    panic!("room not found");
                }
            }
            let mut students_group_indexes = Vec::new();
            for i in record[4].split(",") {
                if let Some(add) = student_groups.iter().position(|x| x.name == i) {
                    students_group_indexes.push(add as usize);
                } else {
                    panic!("student_group not found: {}", i);
                }
            }
            let num_of_students = record[5].parse::<usize>().unwrap();
            let serial_size = record[6].parse::<usize>().unwrap();
            classes.push(class::Class {
                id,
                index,
                num_of_students,
                name,
                teacher_indexes,
                room_candidates_indexes,
                students_group_indexes,
                serial_size,
            });
        }
        Ok(classes)
    }

    pub fn get_classes(&self) -> &Vec<class::Class> {
        &self.classes
    }
    pub fn get_rooms(&self) -> &Vec<room::Room> {
        &self.rooms
    }
    pub fn get_student_groups(&self) -> &Vec<student_group::StudentGroup> {
        &self.student_groups
    }
    pub fn get_teachers(&self) -> &Vec<teacher::Teacher> {
        &self.teachers
    }
}

pub struct InputManager {
    pub input: Mutex<Option<Input>>,
}

#[tauri::command]
pub fn handle_set_input(input_manager: tauri::State<'_, InputManager>) -> Result<(), String> {
    info!("called handle_set_input");
    let input = Input::new();
    let mut managed_input = input_manager.input.lock().unwrap();
    *managed_input = Some(input);
    Ok(())
}

#[tauri::command]
pub fn handle_get_rooms(
    input_manager: tauri::State<'_, InputManager>,
) -> Result<Vec<String>, String> {
    info!("called handle_get_rooms");
    let input = input_manager.input.lock().unwrap();
    if let Some(input) = input.as_ref() {
        return Ok(input.get_rooms().iter().map(|x| x.name.clone()).collect());
    }
    return Err("no input".to_string());
}
