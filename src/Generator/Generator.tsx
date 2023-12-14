import Input from "./Input/Input";
import {invoke} from "@tauri-apps/api/tauri";
import Grid from "./Grid/Grid";
import {useState} from "react";
interface GeneratorProps {
  tableNames: string[];
}

class TimeTable{
  cell_name: number[][]=[];
  pheromone: number[][]= [];
}

const Generator: React.FC<GeneratorProps> = ({ tableNames }) => {
const [input, setInput] = useState<Input | null>(null);
  let [timeTable,setTimeTable] = useState([["NoData"]] as string[][]);
  const sendClassData = () => {
    let json: { [key: string]: any } = {};
    for (const name of tableNames) {
      const data = sessionStorage.getItem(name + "data");
      if (data) {
        json[name+"data"] = JSON.parse(data);
      }
      const columns = sessionStorage.getItem(name + "columns");
      if (columns) {
        json[name+"columns"] = JSON.parse(columns);
      }

    }
    setInput(new Input(json, tableNames));
    console.log("call handle_input");
    invoke("handle_set_input", { input });
  };
  const generate = () => {
    invoke("handle_adapt_input");
  }
  const run_once = () => {
    invoke("handle_aco_run_once").then((res )=>{
      let newTimeTalbe = [] as string[][];
      res = res as TimeTable;
      for (const row of res["cell_name"] as number[][]) {
        let newRow = [] as string[];
        for (const cell of row) {
          if(input){
            newRow.push(cell!=-1?input?.GetClassName(cell):"");
          }else{
            console.log("input is null");
          }
        }
        newTimeTalbe.push(newRow);
      }
      console.log(JSON.stringify(newTimeTalbe));
      setTimeTable(newTimeTalbe)

    }).catch((err)=>{ console.log(err) });
  }


  return (
    <div>
      <button onClick={sendClassData}></button>
      <button onClick={generate}></button>
      <button onClick={run_once}></button>
      <Grid data={timeTable}/>
    </div>
  );
};

export default Generator;
