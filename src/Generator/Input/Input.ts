
export class Input {
    classes: Class[]
    rooms: Room[]
    teachers: Teacher[]
    student_groups: StudentsGroup[]

    constructor(input_json:{[key:string]:any},table_names:string[]) {
        //this.lectures = this.GetLectures(this.table[lecture_table])
        this.teachers = this.GetTeachers(input_json[table_names[1]+"data"],input_json[table_names[1]+"columns"]);
        this.rooms = this.GetRooms(input_json[table_names[2]+"data"],input_json[table_names[2]+"columns"])
        this.student_groups = this.GetStudentGroups(input_json[table_names[3]+"data"],input_json[table_names[3]+"columns"])
        this.classes = this.GetClasses(input_json[table_names[0]+"data"],input_json[table_names[0]+"columns"])
    }


    GetClasses(data:{}[],columns:{}[]): Class[] {
        let res = new Array<Class>
        let idx = 0;
        for(const row of data){
            res.push(new Class(row,columns,idx++,this.teachers,this.rooms,this.student_groups));
        }
        return res
    }

    GetClassName(idx:number):string{
        return this.classes[idx].name
    }
    GetTeacherNames(idx:number):string{
        let res = "";
        for(const teacher_idx of this.classes[idx].teacher_indexes){
            res += this.teachers[teacher_idx].name + ","
        }
        return res;
    }

    GetRooms(data:{}[],columns:{}[]): Room[] {
        var res = new Array<Room>
        let idx = 0;
        for(const row of data){
            res.push(new Room(row,columns,idx++));
        }
        return res
    }

    GetTeachers(data:{}[],columns:{}[]): Teacher[] {
        let res = new Array<Teacher>
        let idx = 0;
        for(const row of data){
            res.push(new Teacher(row,columns,idx++));
        }
        return res
    }

    GetStudentGroups(data:{}[],columns:{}[]): StudentsGroup[] {
        var res = new Array<StudentsGroup>
        let idx = 0;
        for(const row of data){
            res.push(new StudentsGroup(row,columns,idx++));
        }
        return res
    }

    /*
    GenerateIds(id_name_dict: {}, name_id_dict: {}, array: Array<Room | Teacher | StudentsGroup>): void {
        for (const obj of array) {
            id_name_dict[Number(obj.id)] = obj
            name_id_dict[obj.name] = Number(obj.id)
        }
}
*/
}
export class Class {
    id: number;
    index:number;
    num_of_students: number;
    name: string;
    teacher_indexes: Array<number>
    room_candidates_indexes: Array<number>
    students_group_indexes: Array<number>
    constructor(row:{[key:string]:string},columns:{[key:string]:string}[],index:number
        ,teachers:Teacher[]
        ,rooms:Room[]
        ,students_groups:StudentsGroup[]) {
        this.index = index
        this.id = Number(row[columns[0]["accessor"]])
        this.name = row[columns[1]["accessor"]]
        this.teacher_indexes = new Array<number>
        for (const teacher of (row[columns[2]["accessor"]]).split(',')) {
            this.teacher_indexes.push(teachers.findIndex((val)=>val.name==teacher));
        }
        this.room_candidates_indexes = new Array<number>
        for (const room of (row[columns[3]["accessor"]]).split(',')) {
            this.room_candidates_indexes.push(rooms.findIndex((val)=>val.name==room))
        }
        this.students_group_indexes = new Array<number>
        for (const student_group of (row[columns[4]["accessor"]]).split(',')) {
            this.students_group_indexes.push(students_groups.findIndex((val)=>val.name==student_group))

        }
        this.num_of_students = Number(row[columns[5]["accessor"]])
    }
}

/*
const student_group_constructure_table = "学生グループ制約"
export function make_students_group_table(input: Input, sheetHead: GoogleAppsScript.Spreadsheet.Spreadsheet) {
    const len = input.student_groups.length
    let add_table = new Array<Array<String>>(len + 1)
    for(let i = 0;i < len+1;i++){
        add_table[i] = new Array<String>(len+2)
    }
    for(let i = 0;i<len;i++){
        add_table[i+1][0] = input.student_groups[i].name
        add_table[i+1][1] = input.student_groups[i].id.toString()
        add_table[0][i+2] = input.student_groups[i].id.toString()
        add_table[i+1][i+2] = 'x'
    }
    sheetHead.getSheetByName(student_group_constructure_table)
        ?.getRange(1,1,add_table.length,add_table[0].length)
        .setValues(add_table)
}
*/


class Room {
    id: number;
    index: number;
    name: string;
    capacity: number;
    constructor(row:{[key:string]:string},columns:{[key:string]:string}[],index:number) {
        this.index = index
        this.id = Number(row[columns[0]["accessor"]])
        this.name = row[columns[1]["accessor"]]
        this.capacity = Number(row[columns[2]["accessor"]])
    }
}

class Teacher {
    id: number
    index: number
    name: string
    constructor(row: {[key:string]:string},columns:{[key:string]:string}[],index:number) {
        this.index = index
        this.id = Number(row[columns[0]["accessor"]])
        this.name = row[columns[1]["accessor"]]
    }
}
class StudentsGroup {
    id: number
    name: string
    index:number
    constructor(row: {[key:string]:string},columns:{[key:string]:string}[],index:number) {
        this.index = index
        this.id = Number(row[columns[0]["accessor"]])
        this.name = row[columns[1]["accessor"]]
    }
}    
export default Input;