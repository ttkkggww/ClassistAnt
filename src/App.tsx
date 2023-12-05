// App.js

import { useState } from "react";
import CsvTable from "./TableEditor/TableEditor"; // CsvTable コンポーネントのインポート
import styles from "./App.module.css";
import './react-tab.css'
import { Tab, Tabs, TabList, TabPanel } from "react-tabs";

function App() {

  const [tableData, setTableData] = useState<
    Array<Papa.ParseResult<unknown> | null>>
    (
      [null,null,null,null]
      );
  const tableName = ["講義一覧","教員一覧","教室一覧","学生グループ一覧"];
  const handleTableDataLoad = (
    index: number,
    newData: Papa.ParseResult<unknown>
  ) => {
    const newDataArray = [...tableData];
    newDataArray[index] = newData;
    setTableData(newDataArray);
  };
  return (
    <div className={styles.app}>
      <Tabs >
        <TabList>
          {tableData.map((_, index) => (
            <Tab>
              {tableName[index]}
            </Tab>
          ))}
        </TabList>
        {tableData.map((data, index) => (
          <TabPanel key={index}>
            {/* 各タブに CsvTable コンポーネントを配置 */}
            <CsvTable data={data} onDataLoad={(newData) => handleTableDataLoad(index, newData)} />
          </TabPanel>
        ))}
      </Tabs>
    </div>
  );
}

export default App;
