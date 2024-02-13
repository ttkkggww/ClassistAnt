use serde::{Deserialize, Serialize};

use std::error::Error;

use self::{column::Column, student_group::StudentGroup, teacher::Teacher};

pub mod class;
pub mod room;
mod student_group;
mod teacher;
mod column;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Input {
    classes: Vec<class::Class>,
    class_columns: Vec<column::Column>,
    rooms: Vec<room::Room>,
    room_columns: Vec<column::Column>,
    student_groups: Vec<student_group::StudentGroup>,
    student_group_columns: Vec<column::Column>,
    teachers: Vec<teacher::Teacher>,
    teacher_columns: Vec<column::Column>,
}

const TEACHERS_CSV_PATH : &str = "./csvdata/teachers.csv";
const STUDENT_GROUPS_CSV_PATH : &str = "./csvdata/student_groups.csv";
const CLASSES_CSV_PATH : &str = "./csvdata/classes.csv";
const ROOMS_CSV_PATH : &str = "./csvdata/rooms.csv";

impl Input{
    pub fn new () -> Input{
        let (teachers,teacher_columns)= Input::read_teachers_from_csv(&TEACHERS_CSV_PATH.to_string()).unwrap();
        let (rooms,room_columns)= Input::read_rooms_from_csv(&ROOMS_CSV_PATH.to_string()).unwrap();
        let (student_groups,student_group_columns ) = Input::read_student_groups_from_csv(&STUDENT_GROUPS_CSV_PATH.to_string()).unwrap();
        let (classes ,class_columns) = Input::read_classes_from_csv(&CLASSES_CSV_PATH.to_string(),&teachers,&rooms,&student_groups).unwrap();
        Input{classes,class_columns,rooms,room_columns,student_groups,student_group_columns,teachers,teacher_columns}

    }

    fn read_teachers_from_csv(file_path:&String) -> Result<(Vec<teacher::Teacher>,Vec<column::Column>),Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_path(file_path)?;
        let mut columns = Vec::<column::Column>::new();
        let mut teachers = Vec::new();
        let first_record = rdr.records().next().unwrap()?;
        columns.push(column::Column{header:first_record[0].to_string(),accessor:"id".to_string()});
        columns.push(column::Column{header:first_record[1].to_string(),accessor:"name".to_string()});
       for (index,result) in rdr.records().enumerate() {
            let record = result?;
            let id = record[0].parse::<u64>().unwrap();
            let name = record[1].to_string();
            let index = index as u64;
            teachers.push(teacher::Teacher{id,index,name});
        }

        Ok((teachers,columns))
    }

    fn read_rooms_from_csv(file_path:&String) -> Result<(Vec<room::Room>,Vec<column::Column>),Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_path(file_path)?;
        let mut columns = Vec::<column::Column>::new();
        let mut rooms = Vec::new();
        let first_record = rdr.records().next().unwrap()?;
        columns.push(column::Column{header:first_record[0].to_string(),accessor:"id".to_string()});
        columns.push(column::Column{header:first_record[1].to_string(),accessor:"name".to_string()});
        columns.push(column::Column{header:first_record[2].to_string(),accessor:"capacity".to_string()});
        for (index,result) in rdr.records().enumerate() {
            let record = result?;
            let index = index as u64;
            let id = record[0].parse::<u64>().unwrap();
            let name = record[1].to_string();
            let capacity = record[2].parse::<u64>().unwrap();
            rooms.push(room::Room{id,index,name,capacity});
        }
        Ok((rooms,columns))
    }

    fn read_student_groups_from_csv(file_path:&String) -> Result<(Vec<student_group::StudentGroup>,Vec<column::Column>),Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_path(file_path)?;
        let mut columns = Vec::<column::Column>::new();
        let mut student_groups = Vec::new();
        let first_record = rdr.records().next().unwrap()?;
        columns.push(column::Column{header:first_record[0].to_string(),accessor:"id".to_string()});
        columns.push(column::Column{header:first_record[1].to_string(),accessor:"name".to_string()});
        for (index,result) in rdr.records().enumerate() {
            let record = result?;
            let id = record[0].parse::<u64>().unwrap();
            let name = record[1].to_string();
            let index = index as u64;
            student_groups.push(student_group::StudentGroup{id,index,name});
        }
        Ok((student_groups,columns))
    }

    fn read_classes_from_csv(file_path:&String,teachers:&Vec<Teacher>,rooms:&Vec<room::Room>,student_groups:&Vec<StudentGroup>) -> Result<(Vec<class::Class>,Vec<column::Column>),Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_path(file_path)?;
        let mut columns = Vec::<column::Column>::new();
        let mut classes = Vec::new();
        let first_record = rdr.records().next().unwrap()?;
        columns.push(column::Column{header:first_record[0].to_string(),accessor:"id".to_string()});
        columns.push(column::Column{header:first_record[1].to_string(),accessor:"name".to_string()});
        columns.push(column::Column{header:first_record[2].to_string(),accessor:"teachers".to_string()});
        columns.push(column::Column{header:first_record[3].to_string(),accessor:"candidate_rooms".to_string()});
        columns.push(column::Column{header:first_record[4].to_string(),accessor:"student_groups".to_string()});
        columns.push(column::Column{header:first_record[5].to_string(),accessor:"num_of_students".to_string()});
        
        for (index ,result) in rdr.records().enumerate() {
            let record = result?;
            let index = index as u64;
            let id = record[0].parse::<u64>().unwrap();
            let name = record[1].to_string();
            let mut teacher_indexes = Vec::new();
            for i in record[2].split(","){
                if let Some(add) = teachers.iter().position(|x| x.name== i){
                    teacher_indexes.push(add as u64);
                } else {
                    panic!("teacher not found");
                }
            }
            let mut room_candidates_indexes = Vec::new();
            for i in record[3].split(","){
                if let Some(add) = rooms.iter().position(|x| x.name== i){
                    room_candidates_indexes.push(add as u64);
                }else {
                    panic!("room not found");
                }
            }
            let mut students_group_indexes = Vec::new();
            for i in record[4].split(","){
                if let Some(add) = student_groups.iter().position(|x| x.name== i){
                    students_group_indexes.push(add as u64);
                }else {
                    panic!("student_group not found");
                }
            }
            let num_of_students = record[5].parse::<u64>().unwrap();
            classes.push(class::Class{id,index,num_of_students,name,teacher_indexes,room_candidates_indexes,students_group_indexes});
        }
        Ok((classes,columns))
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

    #[allow(dead_code)]
    fn read_csv(file_path:&String) -> Result<(),Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path(file_path)?;
        for result in rdr.records() {
            let record = result?;
            println!("{:?}", record);
        }
        Ok(())
    }
    
}