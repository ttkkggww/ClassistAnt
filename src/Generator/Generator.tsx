import Input from "./Input/Input";
import { invoke } from "@tauri-apps/api/tauri";
import Grid from "./Grid/Grid";
import { useState } from "react";
import  {TimeTable}  from "./Grid/Grid";

interface GeneratorProps {
  tableNames: string[];
}


const Generator: React.FC<GeneratorProps> = ({ tableNames }) => {
  const [input, setInput] = useState<Input | null>(null);
  let [timeTable, setTimeTable] = useState({});
  const sendClassData = () => {
    invoke("handle_set_input");
  };
  const generate = () => {
    invoke("handle_adapt_input");
  };
  const run_once = () => {
    invoke<TimeTable>("handle_aco_run_once")
      .then((res) => {
          console.log(res);
          setTimeTable(res);
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
    </div>
  );
  //<Grid data={timeTable.cell_name} pheromone_256={timeTable.pheromone_256} messages={timeTable.violations_messages} classIds={timeTable.classIds}/>
};

export default Generator;
