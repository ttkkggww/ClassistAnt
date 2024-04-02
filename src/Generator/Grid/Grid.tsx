import React, { useLayoutEffect, useState, useMemo } from "react";
import {invoke} from "@tauri-apps/api/tauri";
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
  period: number;
  room:number;
  isVisible: boolean;
  constructor(id: number, period: number, room: number) {
    this.id = id;
    this.period = period;
    this.room = room;
    this.isVisible = false;
  }
  size?: number;
}
function isActiveCell(cell: Cell): cell is { activeCell: ActiveCell } {
  return (cell as { activeCell: ActiveCell }).activeCell !== undefined;
}

type Cell = { activeCell: ActiveCell } | { blankCell: BlankCell };
export interface TimeTable {
  cells: Array<Cell>;
}

interface GridProps {
  timeTable: TimeTable;
  setTimeTable: (
    timeTable: TimeTable | ((prevTimeTable: TimeTable) => TimeTable),
  ) => void;
}

const GridComponent: React.FC<GridProps> = ({ timeTable, setTimeTable }) => {
  const { cells } = timeTable;

  const handleDragEnd = (event: any) => {
    const { over, active } = event;
    if (over == null) {
      return;
    }
    invoke<TimeTable>("handle_lock_cell",{overId:Number(over.id),activeId:Number(active.id)})
    .then((res)=>{
      setTimeTable(res);
    }).catch((err)=>{
      console.log(err);
    });
  };

  return (
    <div style={{ width: "100%" }}>
      <DndContext onDragEnd={handleDragEnd}>
        <div className={styles["grid-container"]} style={{}}>
          {cells.map((cell, index) => {
            if (isActiveCell(cell)) {
              let cellData = cell.activeCell;
              return (
                <Draggable
                  key = {index}
                  hex_color={cellData.color ?? "#ffffff"}
                  text={cellData.className}
                  id={cellData.id}
                  classId={cellData.id}
                  styles={styles["grid-cell"]}
                  grid_size={cellData.size ?? 1}
                  setTimeTable={setTimeTable}
                />
              );
            }else if(cell.blankCell.isVisible){
              let cellData = cell.blankCell;
              return <Droppable key={index} 
              id={cellData.id} 
              styles={styles["grid-cell"]} 
              grid_size={cellData.size??1} />;
            }
          })}
        </div>
      </DndContext>
      <div></div>
    </div>
  );
};

export default GridComponent;
