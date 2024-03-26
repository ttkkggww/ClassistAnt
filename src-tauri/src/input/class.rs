use serde::{Deserialize, Serialize};
use std::{error::Error, io, process};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Class {
    pub id: usize,
    pub index: usize,
    pub num_of_students: usize,
    pub name: String,
    pub teacher_indexes: Vec<usize>,
    pub room_candidates_indexes: Vec<usize>,
    pub students_group_indexes: Vec<usize>,
    pub serial_size: usize
}

impl Class {
    pub fn get_num_of_students(&self) -> usize {
        self.num_of_students
    }
    pub fn get_teacher_indexes(&self) -> &Vec<usize> {
        &self.teacher_indexes
    }
    pub fn get_students_group_indexes(&self) -> &Vec<usize> {
        &self.students_group_indexes
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
