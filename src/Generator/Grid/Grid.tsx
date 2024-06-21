import React, { useLayoutEffect, useState, useMemo } from "react";
import {invoke} from "@tauri-apps/api/tauri";
import styles from "./Grid.module.css";
import { Droppable } from "./Droppable/Droppable";
import { Draggable } from "./Draggable/Draggable";
import { RoomIndex } from "./RoomIndex/RoomIndex";
import { Period } from "./Period/Period";
import { DndContext, MouseSensor, PointerSensor, useSensors } from "@dnd-kit/core";
import { useSensor } from "@dnd-kit/core";

interface Violations {
  period: number;
  rooms: number[];
}

interface cellsViolations {
  isViolated: boolean;
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
  constructor(id: number,room:number,period:number ,className: string,toolTimeMessage:string) {
    this.id = id;
    this.className = className;
    this.room = room;
    this.period = period;
    this.toolTipMessage = toolTimeMessage;
  }
  teachers?: string[];
  students?: string[];
  color?: string;
  isLocked?: boolean;
  size?: number;
  violations?: cellsViolations;
  toolTipMessage: string;
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

export interface TimeTable {
  classList : (ActiveCell| null)[],
  roomSize: number,
  periodSize: number
}


interface GridProps {
  timeTable: TimeTable;
  setTimeTable: (
    timeTable: TimeTable | ((prevTimeTable: TimeTable) => TimeTable),
  ) => void;
  rooms: string[];
  periods: string[];
}

const Grid: React.FC<GridProps> = ({ timeTable, setTimeTable ,rooms,periods}) => {
  console.log(timeTable)
  const { classList } = timeTable;
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

  return (
    <div style={{ width: "100%" }}>
      <div className={styles["grid-container"]} style={{}}>
      <DndContext onDragEnd={handleDragEnd} onDragOver={handleDragOver} sensors={sensors}>
          {
            classList.map((cell, index) => {
              if (cell!=null) {
                let tipMessage = "";
                if (cell.violations) {
                  if (cell.violations.isViolated) {
                    tipMessage = "This cell has some violations";
                    console.log(cell.violations)
                    if (cell.violations.sameStudentSameTime.length > 0) {
                      tipMessage += "\nSame student in different rooms at the same time";
                      tipMessage += cell.violations.sameStudentSameTime.map((violation) => {
                        return "\nPeriod: " + violation.period + " Rooms: " + violation.rooms.join(",");
                      });
                    }
                    if (cell.violations.sameTeacherSameTime.length > 0) {
                      tipMessage += "\nSame teacher in different rooms at the same time";
                      tipMessage += cell.violations.sameTeacherSameTime.map((violation) => {
                        return "\nPeriod: " + violation.period + " Rooms: " + violation.rooms.join(",");
                      });
                    }
                    if (cell.violations.capacityOver.length > 0) {
                      tipMessage += "\nCapacity is over in some rooms";
                      tipMessage += cell.violations.capacityOver.map((violation) => {
                        return "\nPeriod: " + violation.period + " Rooms: " + violation.rooms.join(",");
                      });
                    }
                    if (cell.violations.strabbleDays.length > 0) {
                      tipMessage += "\nSame class in different days";
                      tipMessage += cell.violations.strabbleDays.map((violation) => {
                        return "\nPeriod: " + violation.period + " Rooms: " + violation.rooms.join(",");
                      });
                    }
                  }
                }
                return (
                  <Draggable 
                    hex_color={cell.color?cell.color:"#ffffff"}
                    text={cell.className + cell.teachers?.join(",") }
                    id={cell.id} 
                    styles={styles["grid-cell"]}
                    room={cell.room}
                    period={cell.period}
                    grid_size={cell.size!}
                    setTimeTable={setTimeTable}
                    isViolated={cell.violations?.isViolated!}
                    toolTipMessage={tipMessage}
                  />
                );
              }
            })
          }
          {
            Array(timeTable.roomSize*timeTable.periodSize).fill(0).map((_,index)=>{
              return (
                <Droppable
                  id={index}
                  styles={styles["grid-cell"]}
                  room={Math.floor(index / timeTable.periodSize)}
                  period={index % timeTable.periodSize}
                  grid_size={1}
                  overColor={overColor}
                />
              )
            })
          }
          
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

export default Grid;
