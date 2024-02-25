import React, { useState } from "react";
import { Column, useTable } from "react-table";
import table_style from "./Table.module.css";

interface TableProp {
  tableName: string;
  columns: Column<any>[];
  data: any[];
  width: number;
}

/**
 * テーブルコンポーネント
 * @param TableProp 引数
 * @returns コンポーネント
 */
function Table({ columns, data }: TableProp) {
  console.log(columns, data);
  // react-tableの定義
  const { getTableProps, getTableBodyProps, headerGroups, rows, prepareRow } =
    useTable(
      {
        columns,
        data,
      },
      // 任意のフックやオプションを追加できます
    );

  return (
    <table {...getTableProps()} className={table_style.table}>
      <thead>
        {headerGroups.map((headerGroup) => (
          <tr {...headerGroup.getHeaderGroupProps()}>
            {headerGroup.headers.map((column) => (
              <th {...column.getHeaderProps()}>{column.render("Header")}</th>
            ))}
          </tr>
        ))}
      </thead>
      <tbody {...getTableBodyProps()}>
        {rows.map((row) => {
          prepareRow(row);
          return (
            <tr {...row.getRowProps()}>
              {row.cells.map((cell) => {
                return <td {...cell.getCellProps()}>{cell.value}</td>;
              })}
            </tr>
          );
        })}
      </tbody>
    </table>
  );
}

export default Table;
