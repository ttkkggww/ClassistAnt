import React, { useState } from "react";
import { Column, useTable } from "react-table";
import table_style from "./Table.module.css";

interface EditableCellProps {
  value: any;
  row: any;
  column: Column<object>;
  updateData: (rowIndex: number, columnId: string|undefined, value: any) => void;
}

const EditableCell: React.FC<EditableCellProps> = ({
  value: initialValue,
  row: { index },
  column: { id },
  updateData,
}) => {
  const [value, setValue] = useState(initialValue);
  const [isEditing, setIsEditing] = useState(false);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setValue(e.target.value);
  };

  const handleBlur = () => {
    setIsEditing(false);
    updateData(index, id, value);
  };

  return isEditing ? (
    <input
      type="text"
      value={value}
      onChange={handleInputChange}
      onBlur={handleBlur}
    />
  ) : (
    <div onClick={() => setIsEditing(true)}>{value}</div>
  );
};

interface TableProp {
  tableName:string;
  columns: Column<object>[];
  data: any[];
  width: number;
}

/**
 * テーブルコンポーネント
 * @param TableProp 引数
 * @returns コンポーネント
 */
function Table({ columns, data ,tableName}: TableProp) {
  const defaultColumn = React.useMemo(
    () => ({
      width: 300,
    }),
    []
  );

  // react-tableの定義
  const { getTableProps, getTableBodyProps, headerGroups, rows, prepareRow } =
    useTable(
      {
        columns,
        data,
        defaultColumn,
      },
      // 任意のフックやオプションを追加できます
    );

  // データの更新関数
  const updateData = (rowIndex: number, columnId: string|undefined, value: any) => {
    if(columnId!==undefined)
    data[rowIndex][columnId] = value;
    sessionStorage.setItem(tableName+data,JSON.stringify(data));
    console.log(`Row ${rowIndex}, Column ${columnId} updated with value: ${value}`);
  };

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
                return (
                <td {...cell.getCellProps()}>
                  {cell.column.id !== "actions" ? (
                    // ここでセルが編集可能かどうかを判定し、編集可能なら EditableCell を使用
                    <EditableCell
                      value={cell.value}
                      row={row}
                      column={cell.column}
                      updateData={updateData}
                    />
                  ) : null}
                </td>
                  
        )})}
            </tr>
          );
        })}
      </tbody>
    </table>
  );
}

export default Table;
