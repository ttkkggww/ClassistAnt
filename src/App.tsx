// App.js

import { useState } from "react";
import TableEditor from "./TableEditor/TableEditor"; // CsvTable コンポーネントのインポート
import styles from "./App.module.css";
import './react-tab.css'
import { Tab, Tabs, TabList, TabPanel } from "react-tabs";
import {Column} from "react-table"
type TableData = {
  columns: Column<object>[];
  data:any[]
}

function App() {
  const tableName = ["講義一覧","教員一覧","教室一覧","学生グループ一覧"];
  return (
    <div className={styles.app}>
      <Tabs >
        <TabList>
          {tableName.map((name) => (
            <Tab>
              {name}
            </Tab>
          ))}
        </TabList>
          {tableName.map((name) => (
          <TabPanel key={name}>
            {/* 各タブに CsvTable コンポーネントを配置 */}
            <TableEditor tableName={name}/>
          </TabPanel>
        ))}
      </Tabs>
    </div>
  );
}

export default App;
