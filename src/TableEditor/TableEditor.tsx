// CsvTable.js

import  { ChangeEvent, useMemo, useState,useEffect } from "react";
import Papa from "papaparse";
import Table from "./Table/Table";
import styles from "../App.module.css";

interface TableEditorProps{
    data: Papa.ParseResult<unknown> |null;
    onDataLoad: (newData: Papa.ParseResult<unknown>) => void;
}

const TableEditor :React.FC<TableEditorProps> = ({data,onDataLoad}) => {
  // CSVから読み込んだデータ
  const [csvData, setCsvData] = useState<Papa.ParseResult<unknown> | null>(null);

  useEffect(() => {
    setCsvData(data)
  },[data])

  // ファイルを選択
  const handleChange = (e:ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (!files || files.length === 0) return;

    // CSVをパース
    Papa.parse(files[0], {
      beforeFirstChunk: function (chunk) {
        var lines = chunk.split("\n");
        for (var i = 0; i < lines.length; i++) {
          var columns = lines[i].split(",");
          var filteredColumns = columns.filter(function (column) {
            return column.trim() !== "";
          });
          lines[i] = filteredColumns.join(",");
        }
        return lines.join("\n");
      },
      complete: function (results) {
        console.log(results);
        setCsvData(results);
        onDataLoad(results);
      },
      skipEmptyLines: true,
    });
  };

  // テーブルに表示する列の定義（CSVの１行目から作成）
  const columns = useMemo(() => {
    if (csvData == null || csvData.data.length === 0) {
      return [{ Header: "No Data" }];
    }
    const row = csvData.data[0] as Array<any>;
    return row.map((cellData, columnIndex) => {
      return {
        Header: cellData,
        accessor: (row: any) => row[columnIndex],
      };
    });
  }, [csvData]);

  return (
    <div className={styles.field}>
      <div style={{ height: "100vh", overflow: "scroll" }}>
        <div style={{ height: "100%" }}>
          <Table columns={columns} data={csvData?.data.slice(1) ?? []} width={660} />
        </div>
      </div>
      <div className={styles.input}>
        <input type="file" onChange={handleChange} />
      </div>
    </div>
  );
};

export default TableEditor;
