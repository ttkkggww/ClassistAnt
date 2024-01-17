import Input from "./Input/Input";
import { invoke } from "@tauri-apps/api/tauri";
import Grid from "./Grid/Grid";
import { useState } from "react";
interface GeneratorProps {
  tableNames: string[];
}

class Violations{
  period:number;
  room_idxes:number[];
  constructor(period:number,room_idxes:number[]){
    this.period = period;
    this.room_idxes = room_idxes;
  }
}
class TimeTable {
  cell_name: number[][] = [];
  pheromone_256: number[][] = [];
  same_teachers_violations: Violations[];
  same_group_violations: Violations[];
  capacity_violations: Violations[];
  constructor(cell_name: number[][], pheromone_256: number[][],same_teachers_violations: Violations[],same_group_violations: Violations[],capacity_violations: Violations[]) {
    this.cell_name = cell_name;
    this.pheromone_256 = pheromone_256;
    this.same_teachers_violations = same_teachers_violations;
    this.same_group_violations = same_group_violations;
    this.capacity_violations = capacity_violations;
  }
}

class DisplayTable{
  cell_name: string[][] = [];
  pheromone_256: number[][] = [];
  violations_messages: string[];
  constructor(cell_name: string[][], pheromone_256: number[][],violations_messages: string[]) {
    this.cell_name = cell_name;
    this.pheromone_256 = pheromone_256;
    this.violations_messages = violations_messages;
  } 
}

const Generator: React.FC<GeneratorProps> = ({ tableNames }) => {
  const [input, setInput] = useState<Input | null>(null);
  let [timeTable, setTimeTable] = useState(new DisplayTable([["NoData"]],[[255]],[]));
  const sendClassData = () => {
    let json: { [key: string]: any } = {};
    for (const name of tableNames) {
      const data = sessionStorage.getItem(name + "data");
      if (data) {
        json[name + "data"] = JSON.parse(data);
      }
      const columns = sessionStorage.getItem(name + "columns");
      if (columns) {
        json[name + "columns"] = JSON.parse(columns);
      }
    }
    let input = new Input(json, tableNames);
    setInput(input);
    invoke("handle_set_input", { input });
  };
  const generate = () => {
    invoke("handle_adapt_input");
  };
  const run_once = () => {
    invoke<TimeTable>("handle_aco_run_once")
      .then((res) => {
        res = res as TimeTable;
          let newTimeTalbe = [] as string[][];
          for (const row of res.cell_name as number[][]) {
            let newRow = [] as string[];
            for (const cell of row) {
              if (input) {
                newRow.push(cell != -1 ? input?.GetClassName(cell) +"\n" + input?.GetTeacherNames(cell): "");
              } else {
                console.log("input is null");
              }
            }
            newTimeTalbe.push(newRow);
          }
          let displayTable = new DisplayTable(newTimeTalbe,res.pheromone_256,[]);
          for (const violation of res.same_teachers_violations) {
            displayTable.violations_messages.push("Same teacher "+JSON.stringify(violation));
          }
          for(const violation of res.same_group_violations){
            displayTable.violations_messages.push("Same group "+JSON.stringify(violation));
          }
          for(const violation of res.capacity_violations){
            displayTable.violations_messages.push("Capacity "+JSON.stringify(violation));
          }
          setTimeTable(displayTable);
        }
      )
      .catch((err) => {
        console.log(err);
      });
  };

  return (
    <div>
      <button onClick={sendClassData}>convert input</button>
      <button onClick={generate}>set input</button>
      <button onClick={run_once}>next generation</button>
      <Grid data={timeTable.cell_name} pheromone_256={timeTable.pheromone_256} messages={timeTable.violations_messages}/>
    </div>
  );
};

export default Generator;
