const TEACHERS_CSV_PATH: &str = "./csvdata/teachers.csv";
const STUDENT_GROUPS_CSV_PATH: &str = "./csvdata/student_groups.csv";
const CLASSES_CSV_PATH: &str = "./csvdata/themed_research.csv";
const ROOMS_CSV_PATH: &str = "./csvdata/rooms.csv";
use std::error::Error;
mod class;
mod column;
mod room;
mod student_group;
mod teacher;
use class::Class;
use column::Column;
use room::Room;
use serde::{Deserialize, Serialize};
use student_group::StudentGroup;
use teacher::Teacher;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TableType {
    Teachers(Teachers),
    StudentGroups(StudentGroups),
    Classes(Classes),
    Rooms(Rooms),
}

#[tauri::command]
pub fn handle_get_table(table_type: String) -> Result<TableType, String> {
    if table_type == "teachers" {
        let res = Teachers::new().map_err(|e| e.to_string())?;
        return Ok(TableType::Teachers(res));
    } else if table_type == "studentGroups" {
        let res = StudentGroups::new().map_err(|e| e.to_string())?;
        return Ok(TableType::StudentGroups(res));
    } else if table_type == "classes" {
        let res = Classes::new().map_err(|e| e.to_string())?;
        return Ok(TableType::Classes(res));
    } else if table_type == "rooms" {
        let res = Rooms::new().map_err(|e| e.to_string())?;
        return Ok(TableType::Rooms(res));
    }
    return Err("Table type not found".to_string());
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Classes {
    pub columns: Vec<Column>,
    pub data: Vec<Class>,
}

impl Classes {
    pub fn new() -> Result<Classes, Box<dyn Error>> {
        Self::read_csv()
    }

    pub fn read_csv() -> Result<Classes, Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(CLASSES_CSV_PATH)?;
        let mut columns = Vec::<Column>::new();
        let mut classes = Vec::<Class>::new();
        let first_record = rdr.records().next().unwrap()?;
        columns.push(Column {
            header: first_record[0].to_string(),
            accessor: "id".to_string(),
        });
        columns.push(Column {
            header: first_record[1].to_string(),
            accessor: "name".to_string(),
        });
        columns.push(Column {
            header: first_record[2].to_string(),
            accessor: "teachers".to_string(),
        });
        columns.push(Column {
            header: first_record[3].to_string(),
            accessor: "candidate_rooms".to_string(),
        });
        columns.push(Column {
            header: first_record[4].to_string(),
            accessor: "student_groups".to_string(),
        });
        columns.push(Column {
            header: first_record[5].to_string(),
            accessor: "num_of_students".to_string(),
        });
        columns.push(Column {
            header: first_record[6].to_string(),
            accessor: "serial_size".to_string(),
        });
        for result in rdr.records() {
            let record = result?;
            let id = record[0].to_string();
            let name = record[1].to_string();
            let teachers = record[2].to_string();
            let candidate_rooms = record[3].to_string();
            let student_groups = record[4].to_string();
            let num_of_students = record[5].to_string();
            let serial_size = record[6].to_string();
            classes.push(Class {
                id,
                name,
                teachers,
                candidate_rooms,
                student_groups,
                num_of_students,
                serial_size,
            });
        }
        Ok(Classes {
            columns: columns,
            data: classes,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rooms {
    pub columns: Vec<Column>,
    pub data: Vec<Room>,
}

impl Rooms {
    pub fn new() -> Result<Rooms, Box<dyn Error>> {
        Self::read_csv()
    }
    pub fn read_csv() -> Result<Rooms, Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(ROOMS_CSV_PATH)?;
        let mut columns = Vec::<column::Column>::new();
        let mut rooms = Vec::new();
        let first_record = rdr.records().next().unwrap()?;
        columns.push(column::Column {
            header: first_record[0].to_string(),
            accessor: "id".to_string(),
        });
        columns.push(column::Column {
            header: first_record[1].to_string(),
            accessor: "name".to_string(),
        });
        columns.push(column::Column {
            header: first_record[2].to_string(),
            accessor: "capacity".to_string(),
        });
        for result in rdr.records() {
            let record = result?;
            let id = record[0].to_string();
            let name = record[1].to_string();
            let capacity = record[2].to_string();
            rooms.push(Room { id, name, capacity });
        }
        Ok(Rooms {
            columns: columns,
            data: rooms,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentGroups {
    pub columns: Vec<Column>,
    pub data: Vec<StudentGroup>,
}

impl StudentGroups {
    pub fn new() -> Result<StudentGroups, Box<dyn Error>> {
        Self::read_csv()
    }

    pub fn read_csv() -> Result<StudentGroups, Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(STUDENT_GROUPS_CSV_PATH)?;
        let mut columns = Vec::<column::Column>::new();
        let mut student_groups = Vec::new();
        let first_record = rdr.records().next().unwrap()?;
        columns.push(column::Column {
            header: first_record[0].to_string(),
            accessor: "id".to_string(),
        });
        columns.push(column::Column {
            header: first_record[1].to_string(),
            accessor: "name".to_string(),
        });
        for result in rdr.records() {
            let record = result?;
            let id = record[0].to_string();
            let name = record[1].to_string();
            student_groups.push(StudentGroup { id, name });
        }
        Ok(StudentGroups {
            columns: columns,
            data: student_groups,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Teachers {
    pub columns: Vec<Column>,
    pub data: Vec<Teacher>,
}

impl Teachers {
    pub fn new() -> Result<Teachers, Box<dyn Error>> {
        Self::read_csv()
    }
    pub fn read_csv() -> Result<Teachers, Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(TEACHERS_CSV_PATH)?;
        let mut columns = Vec::<column::Column>::new();
        let mut teachers = Vec::new();
        let first_record = rdr.records().next().unwrap()?;
        columns.push(column::Column {
            header: first_record[0].to_string(),
            accessor: "id".to_string(),
        });
        columns.push(column::Column {
            header: first_record[1].to_string(),
            accessor: "name".to_string(),
        });
        for result in rdr.records() {
            let record = result?;
            let id = record[0].to_string();
            let name = record[1].to_string();
            teachers.push(Teacher { id, name });
        }
        Ok(Teachers {
            columns: columns,
            data: teachers,
        })
    }
}
