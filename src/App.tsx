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
  const tableNames = ["classes","teachers","rooms","studentGroups"];
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
            <TableEditor tableName={name}/>
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
