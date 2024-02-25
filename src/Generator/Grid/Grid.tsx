import React, { useLayoutEffect, useState, useMemo } from "react";
import styles from "./Grid.module.css";
import { Droppable } from "./Droppable/Droppable";
import { Draggable } from "./Draggable/Draggable";
import { DndContext } from "@dnd-kit/core";

class ActiveCell {
    id: number;
    className: string;
    constructor(id: number, className: string) {
      this.id = id;
      this.className = className;
    }
    teachers?: string[];
    students?: string[];
    color?: string;
    isLocked?: boolean;
    size?: number;
}

class BlankCell {
    id: number;
    constructor(id: number) {
      this.id = id;
    }
    size?: number;
}
function isActiveCell(cell: Cell): cell is { activeCell: ActiveCell } {
  return (cell as { activeCell: ActiveCell }).activeCell !== undefined;
}

type Cell =
  | { activeCell:ActiveCell }
  | { blankCell:BlankCell };
export interface TimeTable {
    cells: Array<Cell>;
}


interface GridProps {
  timeTable: TimeTable;
  setTimeTable: (timeTable: TimeTable | ((prevTimeTable: TimeTable) => TimeTable)) => void;

}

const GridComponent: React.FC<GridProps> = ({
  timeTable,
  setTimeTable
}) => {

  const {cells} = timeTable;

  const handleDragEnd = (event: any) => {
    const { over, active } = event;
    if(over==null){
      return;
    }
    console.log(over, active);

    const swapGridArray = (index1: number, index2: number) => {
      setTimeTable((prevTimeTable:TimeTable) => {
        const newTimeTable = {...prevTimeTable};
        [newTimeTable.cells[index1], newTimeTable.cells[index2]] = [
          newTimeTable.cells[index2],
          newTimeTable.cells[index1],
        ];
        return newTimeTable as TimeTable;
      });
    };

    swapGridArray(active.id, over.id);
  };


  return (
    <div style={{ width: "100%" }}>
      <DndContext onDragEnd={handleDragEnd}>
        <div className={styles["grid-container"]} style={{}}>
          {cells.map((cell, index) => {
            if (isActiveCell(cell)){
              let cellData = cell.activeCell;
              return (
                <Draggable
                  hex_color={cellData.color??"#ffffff"}
                  text={cellData.className}
                  id={cellData.id}
                  classId={cellData.id}
                  styles={styles["grid-cell"]}
                />
              );
            }
            let cellData = cell.blankCell;
              return <Droppable id={cellData.id} styles={styles["grid-cell"]} />;
          })}
        </div>
      </DndContext>
      <div>
      </div>
    </div>
  );
};

export default GridComponent;
