use serde::{Deserialize, Serialize};

pub mod class;
pub mod room;
mod student_group;
mod teacher;

#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    classes: Vec<class::Class>,
    rooms: Vec<room::Room>,
    student_groups: Vec<student_group::StudentGroup>,
    teachers: Vec<teacher::Teacher>,
}

impl Input{
    pub fn new() -> Input{
        Input{
            classes: Vec::new(),
            rooms: Vec::new(),
            student_groups: Vec::new(),
            teachers: Vec::new(),
        }
    }
    pub fn get_classes(&self) -> &Vec<class::Class>{
        &self.classes
    }
    pub fn get_rooms(&self) -> &Vec<room::Room>{
        &self.rooms
    }
    pub fn get_student_groups(&self) -> &Vec<student_group::StudentGroup>{
        &self.student_groups
    }
    pub fn get_teachers(&self) -> &Vec<teacher::Teacher>{
        &self.teachers
    }
    
}