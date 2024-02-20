import React, { useLayoutEffect, useState, useMemo } from "react";
import styles from "./Grid.module.css";
import { rgbToHex } from "../../modules/color";
import { Droppable } from "./Droppable/Droppable";
import { Draggable } from "./Draggable/Draggable";
import { DndContext } from "@dnd-kit/core";
import { invoke } from '@tauri-apps/api/tauri'

interface Cell {
    className: string;
    teachers?: string[];
    students?: string[];
    color?: string;
    isLocked?: boolean;
    size?: number;
}

export interface TimeTable {
    cells: Array<Cell | undefined>;
}


interface GridProps {
  data: string[][];
  pheromone_256: number[][];
  messages: string[];
  classIds: number[][];
}

const GridComponent: React.FC<GridProps> = ({
  data,
  pheromone_256,
  messages,
  classIds
}) => {
  const generateGridArray = () => {
    let result = [];
    for (let [rowIndex, row] of data.entries()) {
      for (let [columnIndex, cell] of row.entries()) {
        result.push({
          is_empty: cell === "",
          color_hex: rgbToHex(
            255,
            255 - pheromone_256[rowIndex][columnIndex],
            255 - pheromone_256[rowIndex][columnIndex]
          ),
          text: cell,
          id: rowIndex * row.length + columnIndex,
          rowIndex: rowIndex,
          columnIndex: columnIndex,
          classId: classIds[rowIndex][columnIndex]
        });
      }
    }
    return result;
  };

  const [gridArray, setGridArray] = useState(generateGridArray);

  const handleDragEnd = (event: any) => {
    const { over, active } = event;
    if(over==null){
      return;
    }
    console.log(over, active);
    invoke("handle_one_hot_pheromone",{
      classId:gridArray[active.id].classId,
      roomId:gridArray[over.id].rowIndex,
      periodId:gridArray[over.id].columnIndex
    });

    const swapGridArray = (index1: number, index2: number) => {
      setGridArray((prevGridArray) => {
        const newGridArray = [...prevGridArray];
        [newGridArray[index1], newGridArray[index2]] = [
          newGridArray[index2],
          newGridArray[index1],
        ];
        newGridArray[index1].id = index1;
        newGridArray[index2].id = index2;
        return newGridArray;
      });
    };

    swapGridArray(active.id, over.id);
  };

  // useMemo を使って依存配列を安定させる
  const dependencies = useMemo(() => [data, pheromone_256], [data, pheromone_256]);

  // useLayoutEffect の中での state の更新
  useLayoutEffect(() => {
    setGridArray(generateGridArray());
  }, dependencies);

  return (
    <div style={{ width: "100%" }}>
      <DndContext onDragEnd={handleDragEnd}>
        <div className={styles["grid-container"]} style={{}}>
          {gridArray.map((cell, index) => {
            if (cell.is_empty) {
              return <Droppable id={index} styles={styles["grid-cell"]} />;
            }
            return (
              <Draggable
                hex_color={cell.color_hex}
                text={cell.text}
                id={cell.id}
                classId={classIds[cell.rowIndex][cell.columnIndex]}
                styles={styles["grid-cell"]}
              />
            );
          })}
        </div>
      </DndContext>
      <div>
        {messages.map((str, index) => (
          <React.Fragment key={index}>
            {str}
            {index < messages.length - 1 && <br />}
          </React.Fragment>
        ))}
      </div>
    </div>
  );
};

export default GridComponent;
