import Input from "./Input/Input";
import {invoke} from "@tauri-apps/api/tauri";
interface GeneratorProps {
  tableNames: string[];
}

const Generator: React.FC<GeneratorProps> = ({ tableNames }) => {
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
    const input = new Input(json, tableNames);
    console.log("call handle_input");
    invoke("handle_input", { input });
  };
  return (
    <div>
      <button onClick={sendClassData}></button>
    </div>
  );
};

export default Generator;
