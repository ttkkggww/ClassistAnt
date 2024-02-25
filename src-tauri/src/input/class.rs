

use serde::{Deserialize, Serialize};
use std::{error::Error, io , process};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Class {
    pub id: u64,
    pub index: u64,
    pub num_of_students: u64,
    pub name: String,
    pub teacher_indexes: Vec<u64>,
    pub room_candidates_indexes: Vec<u64>,
    pub students_group_indexes: Vec<u64>,
}

impl Class{
    pub fn get_num_of_students(&self) -> u64{
        self.num_of_students
    }
    pub fn get_teacher_indexes(&self) -> &Vec<u64>{
        &self.teacher_indexes
    }
    pub fn get_students_group_indexes(&self) -> &Vec<u64>{
        &self.students_group_indexes
    }
    pub fn get_name(&self) -> &String{
        &self.name
    }

}