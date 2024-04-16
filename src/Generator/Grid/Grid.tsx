import React, { useLayoutEffect, useState, useMemo } from "react";
import {invoke} from "@tauri-apps/api/tauri";
import styles from "./Grid.module.css";
import { Droppable } from "./Droppable/Droppable";
import { Draggable } from "./Draggable/Draggable";
import { RoomIndex } from "./RoomIndex/RoomIndex";
import { Period } from "./Period/Period";
import { DndContext, MouseSensor, PointerSensor, useSensors } from "@dnd-kit/core";
import { useSensor } from "@dnd-kit/core";
let startX:number,startY: number;

interface Violations {
  period: number;
  rooms: number[];
}

interface cellsViolations {
  is_violated: boolean;
  sameStudentSameTime: Violations[];
  sameTeacherSameTime: Violations[];
  capacityOver: Violations[];
  strabbleDays : Violations[];
}
class ActiveCell {
  id: number;
  className: string;
  room: number;
  period: number;
  constructor(id: number,room:number,period:number ,className: string) {
    this.id = id;
    this.className = className;
    this.room = room;
    this.period = period;
  }
  teachers?: string[];
  students?: string[];
  color?: string;
  isLocked?: boolean;
  size?: number;
  violations?: cellsViolations;
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
  rooms: string[];
  periods: string[];
}

const distance = (x1: number, y1: number, x2: number, y2: number) => {
  return Math.sqrt((x1 - x2) ** 2 + (y1 - y2) ** 2);
}

const GridComponent: React.FC<GridProps> = ({ timeTable, setTimeTable ,rooms,periods}) => {
  const { cells } = timeTable;

  const sensor = useSensor(PointerSensor, {
    activationConstraint: {
      distance: 5,
      },
    });
  const sensors = useSensors(sensor);
  const handleDragEnd = (event: any) => {
    const { over, active } = event;
    if (over == null) {
      return;
    }
    let endX = event.activatorEvent.clientX;
    let endY = event.activatorEvent.clientY;
    if(distance(startX,startY,endX,endY) < 5){
      return;
    }
    let is_swappable;
    invoke<boolean>("is_swappable",{overId:Number(over.id),activeId:Number(active.id)}).then((res)=>{
      is_swappable = res;
      if(!is_swappable){
        return;
      }
      invoke<TimeTable>("handle_swap_cell",{overId:Number(over.id),activeId:Number(active.id)})
        .then((res)=>{
          setTimeTable(res);
        }).catch((err)=>{
          console.log(err);
        });
      }).catch((err)=>{
        console.log(err);
    });
  };

  const [overColor, setOverColor] = useState("gray");

  const handleDragOver = (event: any) => {
    const {over,active} = event;
    if(over == null){
      return;
    }
    invoke<boolean>("is_swappable",{overId:Number(over.id),activeId:Number(active.id)}).then((res)=>{
      if(res){
        setOverColor("#5CB85C");
      }else{
        setOverColor("#F0AD4E");
      }
    }).catch((err)=>{
      console.log(err);
    });
  }
  
  console.log('a');

  return (
    <div style={{ width: "100%" }}>
      <div className={styles["grid-container"]} style={{}}>
      <DndContext onDragEnd={handleDragEnd} onDragOver={handleDragOver} sensors={sensors}>
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
                  room={cellData.room}
                  period={cellData.period}
                  grid_size={cellData.size ?? 1}
                  setTimeTable={setTimeTable}
                  isViolated={cellData.violations?.is_violated ?? false}
                />
              );
            }else {
              let cellData = cell.blankCell;
              return <Droppable key={index} 
              id={cellData.id} 
              styles={styles["grid-cell"]} 
              room={cellData.room}
              period={cellData.period}
              grid_size={cellData.size??1} 
              overColor={overColor}/>;

            }
          })}
      </DndContext>
      {rooms.map((room,index)=>{
        return <RoomIndex key={index} id={index} name={room} styles={styles["grid-cell"]}/>
      })}
      {periods.map((period,index)=>{
        return <Period key={index} id={index} name={period} styles={styles["grid-cell"]}/>
      })}
      </div>
    </div>
  );
};

export default GridComponent;
