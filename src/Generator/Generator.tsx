import Input from "./Input/Input";
import { invoke } from "@tauri-apps/api/tauri";
import Grid from "./Grid/Grid";
import { useEffect, useState } from "react";
import { TimeTable } from "./Grid/Grid";

interface GeneratorProps {
  tableNames: string[];
}

const Generator: React.FC<GeneratorProps> = ({ tableNames }) => {
  let [timeTable, setTimeTable] = useState({
    classList: [],
    roomSize: 0,
    periodSize: 0,
  } as TimeTable);
  let [rooms, SetRooms] = useState([] as string[]);
  let [periods, SetPeriods] = useState([] as string[]);
  let [showColor, setShowColor] = useState(true);

  useEffect(() => {
    invoke<string[]>("handle_get_rooms")
      .then((res) => {
        SetRooms(res);
      })
      .catch((err) => {
        SetRooms([err]);
      });

    invoke<string[]>("handle_get_periods")
      .then((res) => {
        SetPeriods(res);
      })
      .catch((err) => {
        SetPeriods([err]);
      });
  }, [timeTable]);

  const sendClassData = () => {
    invoke("handle_set_input");
  };
  const generate = () => {
    invoke("handle_adapt_input");
  };
  const run_once = () => {
    if (timeTable.classList.length != 0) {
      invoke("handle_read_cells", { cells: timeTable.classList });
    }
    invoke<TimeTable>("handle_aco_run_once")
      .then((res) => {
        setTimeTable(res);
      })
      .catch((err) => {
        console.log(err);
      });
  };
  const run_no_violation = () => {
    if (timeTable.classList.length != 0) {
      invoke("handle_read_cells", { cells: timeTable.classList });
    }
    invoke<TimeTable>("handle_aco_run_no_violations")
      .then((res) => {
        setTimeTable(res);
      })
      .catch((err) => {
        console.log(err);
      });
  };

  const save_time_table = () => {
    invoke("dump_timetable");
  };

  const load_time_table = () => {
    invoke<TimeTable>("load_timetable")
      .then((res) => {
        setTimeTable(res);
      })
      .catch((err) => {
        console.log(err);
      });
  };
  const calc_performance = () => {
    invoke("handle_calc_performance")
      .then((res) => {
        console.log(res);
      })
      .catch((err) => {
        console.log(err);
      });
  }
  const handleShowColor = () => {
    setShowColor(!showColor);
  };
  
  const handle_lock_no_violation = () => {
    invoke<TimeTable>("handle_lock_no_violation")
      .then((res) => {
        setTimeTable(res);
      })
      .catch((err) => {
        console.log(err);
      });
  }
  
  const handle_unlock_violation = () => {
    invoke<TimeTable>("handle_unlock_violation")
      .then((res) => {
        setTimeTable(res);
      })
      .catch((err) => {
        console.log(err);
      });
  }

  return (
    <div>
      <button onClick={sendClassData}>入力を変換</button>
      <button onClick={generate}>リセット</button>
      <button onClick={run_once}>時間割を作る</button>
      <button onClick={run_once}>時間割を計算する</button>
      <button onClick={run_no_violation}>成約違反がなくなるまで計算する。</button>
      <button onClick={save_time_table}>save timetable</button>
      <button onClick={load_time_table}>load timetable</button>
      <button onClick={calc_performance}>calc performance</button>
      <button onClick={handle_lock_no_violation}>制約違反以外のコマをロック</button>
      <button onClick={handle_unlock_violation}>制約違反のコマをアンロック</button>
      <label>
        <input
          type="checkbox"
          checked={showColor}
          onChange={handleShowColor}
        ></input>
        Show Color
      </label>
      <Grid
        timeTable={timeTable}
        setTimeTable={setTimeTable}
        rooms={rooms}
        periods={periods}
        showColor={showColor}
      />
    </div>
  );
};

export default Generator;
