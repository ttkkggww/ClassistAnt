// App.js

import { useState } from "react";
import TableEditor from "./TableEditor/TableEditor"; // CsvTable コンポーネントのインポート
import styles from "./App.module.css";
import './react-tab.css'
import { Tab, Tabs, TabList, TabPanel } from "react-tabs";
import {Column} from "react-table"
import Generator from "./Generator/Generator"
type TableData = {
  columns: Column<object>[];
  data:any[]
}

function App() {
  const tableNames = ["講義一覧","教員一覧","教室一覧","学生グループ一覧"];
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

          {tableNames.map((name) => (
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
