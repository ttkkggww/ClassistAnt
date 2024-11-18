import { useDraggable } from "@dnd-kit/core";
import {invoke} from "@tauri-apps/api/tauri";
import { TimeTable } from "../Grid";
import { Tooltip  } from "react-tooltip";
import React from "react";

interface DraggableProps {
  hex_color: string;
  text: string;
  id: number;
  styles: string;
  room: number;
  period: number;
  grid_size: number;
  setTimeTable: (
    timeTable: TimeTable | ((prevTimeTable: TimeTable) => TimeTable),
  ) => void;
  isViolated: boolean;
  toolTipMessage: string;
}

export function Draggable({ hex_color, text, id, styles,room,period,grid_size,setTimeTable ,isViolated,toolTipMessage}: DraggableProps) {
  const { attributes, listeners, setNodeRef, transform } = useDraggable({
    id: id.toString(),
  });
  let x = room + 2;
  let y = period + 2;
  const style = transform
    ? {
        transform: `translate3d(${transform.x}px, ${transform.y}px, 0)`,
        backgroundColor: hex_color,
        gridColumn: `span ${grid_size}`,
        gridArea: `${y}/${x}/${y+grid_size}/${x+1}`,
        zIndex: 3,
        border : isViolated ? '2px solid red' : '',
      }
    : {
        backgroundColor: hex_color,
        gridColumn: `span ${grid_size}`,
        gridArea: `${y}/${x}/${y+grid_size}/${x+1}`,
        zIndex: 2,
        border : isViolated ? '2px solid red' : '',
    };

  const handleDobuleClick = () => {
    invoke<TimeTable>("handle_switch_lock", {id:id})
    .then((res) => {
      setTimeTable(res);
    }).catch((err) => {
      console.log(err);
    });
  }
  console.log(toolTipMessage);
  return (
    <>
      <div ref={setNodeRef} 
        {...listeners} 
        {...attributes}
        onDoubleClick={handleDobuleClick}
        style={style} className={styles}
        data-tooltip-id={id.toString()}
        data-tooltip-content={toolTipMessage}
        >
          {text} 
      </div>
      
      <Tooltip id={id.toString()} style={{zIndex:5}}/>
    </>
  );
}
