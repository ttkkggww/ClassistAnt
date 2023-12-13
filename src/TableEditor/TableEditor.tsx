// CsvTable.js

import  { ChangeEvent, useMemo, useState,useEffect } from "react";
import Papa from "papaparse";
import Table from "./Table/Table";
import styles from "../App.module.css";
import {Column} from "react-table";
import { readTextFile } from "@tauri-apps/api/fs";
interface TableEditorProps{
  tableName:string
  defaultPath:string
}

const TableEditor :React.FC<TableEditorProps> = (props:TableEditorProps) => {
  // CSVから読み込んだデータ
  const [columns, setColumns] = useState<Column<object>[]>(() => {
    // ページを開いたときに sessionStorage から columns 情報を読み込む
    const storedColumns = sessionStorage.getItem(props.tableName + "columns");
    return storedColumns ? JSON.parse(storedColumns) : [{ Header: "No Data" }];
  });
  const [data, setData] = useState<Column<object>[]>(() => {

    const storedData = sessionStorage.getItem(props.tableName + "data");
    return storedData ? JSON.parse(storedData) : [];
  });

  //ここでキャッシュをきかせてる
useEffect(() => {
  // 最新の columns を sessionStorage に保存
  sessionStorage.setItem(
    props.tableName + "data",
    JSON.stringify(data)
  );
  console.log("dataをセットしました。", data);

  // クリーンアップ関数: コンポーネントがアンマウントされるときに実行
  return () => {
    sessionStorage.setItem(
      props.tableName + "data",
      JSON.stringify(data)
    );
    console.log("columnsをセットしました。", data);
  };
}, [data, props.tableName]);


useEffect(() => {
  // 最新の columns を sessionStorage に保存
  sessionStorage.setItem(
    props.tableName + "columns",
    JSON.stringify(columns)
  );
  console.log("columnsをセットしました。", columns);

  // クリーンアップ関数: コンポーネントがアンマウントされるときに実行
  return () => {
    sessionStorage.setItem(
      props.tableName + "columns",
      JSON.stringify(columns)
    );
    console.log("columnsをセットしました。", columns);
  };
}, [columns, props.tableName]);

  useEffect(() => {
    if (props.defaultPath) {
      readTextFile(props.defaultPath).then((fileContent) =>
      Papa.parse(fileContent, {
      beforeFirstChunk: function (chunk) {
        var lines = chunk.split("\n");
        for (var i = 0; i < lines.length; i++) {
          let columns = lines[i].split(",");
          var filteredColumns = columns.filter(function (column) {
            return column.trim() !== "";
          });
          lines[i] = filteredColumns.join(",");
        }
        return lines.join("\n");
      },
        complete: function (results) {
          const row = results.data[0] as Array<any>;
          const newColumns = row.map((cellData) => {
            return {
              Header: cellData,
              accessor: cellData,
            };
          });
          setColumns(newColumns);
          let newData: any[] = [];
          for (let i = 1; i < results.data.length; i++) {
            let addData: any = {};
            let curRow = results.data[i] as Array<any>;
            for (let j = 0; j < curRow.length; j++) {
              addData[row[j]] = curRow[j];
            }
            newData.push(addData);
          }
          setData(newData);
        },
        skipEmptyLines: true,
      })).catch((err)=>{
        console.log(err);
      });
    }
  }, [props.defaultPath]);
  // ファイルを選択
  const handleChange = (e:ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if(!files)return;

    // CSVをパース
    Papa.parse(files[0], {
      beforeFirstChunk: function (chunk) {
        var lines = chunk.split("\n");
        for (var i = 0; i < lines.length; i++) {
          let columns = lines[i].split(",");
          var filteredColumns = columns.filter(function (column) {
            return column.trim() !== "";
          });
          lines[i] = filteredColumns.join(",");
        }
        return lines.join("\n");
      },
      complete: function (results) {
        const row = results.data[0] as Array<any>;
        const newColumns = row.map((cellData) => {
          return {
            Header: cellData,
            accessor: cellData,
          }
        });
        setColumns(newColumns);
        let newData:any[] = [];
        for(let i = 1;i<results.data.length;i++){
          let addData:any = {};
          let curRow = results.data[i] as Array<any>;
          for(let j = 0;j<curRow.length;j++){
            addData[row[j]] = curRow[j]
          }
          newData.push(addData);
        }
        setData(newData);
      },
      skipEmptyLines: true,
    });
  };

  // テーブルに表示する列の定義（CSVの１行目から作成）
  return (
    <div className={styles.field}>
      <div style={{ height: "100vh", overflow: "scroll" }}>
        <div style={{ height: "100%" }}>
            <Table columns={columns} data={data} width={660} tableName={props.tableName} />
        </div>
      </div>
      <div className={styles.input}>
        <input type="file" onChange={handleChange} />
      </div>
    </div>
  );
};

export default TableEditor;
