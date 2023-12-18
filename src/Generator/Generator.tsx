import Input from "./Input/Input";
import { invoke } from "@tauri-apps/api/tauri";
import Grid from "./Grid/Grid";
import { useState } from "react";
interface GeneratorProps {
  tableNames: string[];
}

class TimeTable {
  cell_name: number[][] = [];
  pheromone_256: number[][] = [];
  constructor(cell_name: number[][], pheromone_256: number[][]) {
    this.cell_name = cell_name;
    this.pheromone_256 = pheromone_256;
  } 
}

class DisplayTable{
  cell_name: string[][] = [];
  pheromone_256: number[][] = [];
  constructor(cell_name: string[][], pheromone_256: number[][]) {
    this.cell_name = cell_name;
    this.pheromone_256 = pheromone_256;
  } 
}

const Generator: React.FC<GeneratorProps> = ({ tableNames }) => {
  const [input, setInput] = useState<Input | null>(null);
  let [timeTable, setTimeTable] = useState(new DisplayTable([["NoData"]],[[255]]));
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
    setInput(new Input(json, tableNames));
    console.log("call handle_input");
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
          console.log(JSON.stringify(res))
          setTimeTable(new DisplayTable(newTimeTalbe,res.pheromone_256));
        }
      )
      .catch((err) => {
        console.log(err);
      });
  };

  return (
    <div>
      <button onClick={sendClassData}></button>
      <button onClick={generate}></button>
      <button onClick={run_once}></button>
      <Grid data={timeTable.cell_name} pheromone_256={timeTable.pheromone_256} />
    </div>
  );
};

export default Generator;
