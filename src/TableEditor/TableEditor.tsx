// CsvTable.js

import { useState, useEffect } from "react";
import Table from "./Table/Table";
import styles from "../App.module.css";
import { Column } from "react-table";
import { invoke } from "@tauri-apps/api/tauri";

interface TableEditorProps {
  tableName: string;
}

const TableEditor: React.FC<TableEditorProps> = (props: TableEditorProps) => {
  type TableData = {
    columns: Column<any>[];
    data: any[];
  };

  const defaultTableData = {
    columns: [
      {
        Header: "NoData",
        accessor: "NoData",
      },
    ],
    data: [
      {
        NoData: "Now Loading...",
      },
    ],
  } as TableData;

  const [table, setTable] = useState(defaultTableData);

  useEffect(() => {
    invoke<any>("handle_get_table", { tableType: props.tableName })
      .then((res) => {
        let { columns, data } = Object.keys(res).map((key) => ({
          columns: res[key].columns as any[],
          data: res[key].data as any[],
        }))[0];
        for (let i = 0; i < columns.length; i++) {
          columns[i].Header = columns[i].header;
        }
        setTable({ columns, data });
      })
      .catch((err) => {
        console.log(err, props.tableName);
      });
  }, []);
  return (
    <div className={styles.field}>
      <div style={{ height: "100vh", overflow: "scroll" }}>
        <div style={{ height: "100%" }}>
          <Table
            columns={table.columns}
            data={table.data}
            width={660}
            tableName={props.tableName}
          />
        </div>
      </div>
    </div>
  );
};

export default TableEditor;
