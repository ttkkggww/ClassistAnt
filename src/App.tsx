// App.js

import { useState } from "react";
import TableEditor from "./TableEditor/TableEditor"; // CsvTable コンポーネントのインポート
import styles from "./App.module.css";
import './react-tab.css'
import { Tab, Tabs, TabList, TabPanel } from "react-tabs";
import {Column} from "react-table"
import Generator from "./Generator/Generator"
import {readDir} from "@tauri-apps/api/fs";
type TableData = {
  columns: Column<object>[];
  data:any[]
}

function App() {
  const tableNames = ["講義一覧","教員一覧","教室一覧","学生グループ一覧"];
  const defaultPathes = ["./csvdata/講義一覧.csv","./csvdata/教員一覧.csv","./csvdata/教室一覧.csv","./csvdata/学生グループ一覧.csv"];
  readDir("./csvdata").then((result)=>{
    console.log("csvdataの中身")
    result.forEach((file)=>{
      console.log(file);
    })
  }).catch((err)=>{
    console.log(err);
  })
  return (
    <div className={styles.app}>
      <Tabs >
        <TabList>
          {tableNames.map((name) => (
            <Tab>
              {name}
            </Tab>
          ))}
          <Tab>
            時間割生成
          </Tab>
        </TabList>
          {tableNames.map((name,i) => (
          <TabPanel key={name}>
            {/* 各タブに CsvTable コンポーネントを配置 */}
            <TableEditor tableName={name} defaultPath={defaultPathes[i]}/>
          </TabPanel>
        ))}
        <TabPanel>
          <Generator tableNames = {tableNames}></Generator>
        </TabPanel>
      </Tabs>
    </div>
  );
}

export default App;
