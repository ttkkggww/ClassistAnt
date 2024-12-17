import { useDraggable } from "@dnd-kit/core";
import { invoke } from "@tauri-apps/api/tauri";
import { TimeTable } from "../Grid";
import { Tooltip } from "react-tooltip";
import React, { useCallback } from "react";

interface DraggableProps {
  hex_color: string;
  text: string;
  id: number;
  styles: string;
  room: number;
  period: number;
  grid_size: number;
  setTimeTable: (
    timeTable: TimeTable | ((prevTimeTable: TimeTable) => TimeTable)
  ) => void;
  isViolated: boolean;
  toolTipMessage: string;
  isWorst3: boolean;
}

export function Draggable({
  hex_color,
  text,
  id,
  styles,
  room,
  period,
  grid_size,
  setTimeTable,
  isViolated,
  toolTipMessage,
  isWorst3,
}: DraggableProps) {
  const { attributes, listeners, setNodeRef, transform } = useDraggable({
    id: id.toString(),
  });

  const x = room + 2;
  const y = period + 2;

  const style = {
    transform: transform
      ? `translate3d(${transform.x}px, ${transform.y}px, 0)`
      : undefined,
    backgroundColor: isWorst3 ? "transparent" : hex_color,
    backgroundImage: isWorst3
      ? `repeating-linear-gradient(
          45deg,
          ${hex_color} 0,
          ${hex_color} 10px,
          transparent 10px,
          transparent 20px
        )`
      : undefined,
    gridColumn: `span ${grid_size}`,
    gridArea: `${y}/${x}/${y + grid_size}/${x + 1}`,
    zIndex: transform ? 3 : 2,
    border: isViolated ? "2px solid red" : "",
  };

  const handleDoubleClick = useCallback(() => {
    invoke<TimeTable>("handle_switch_lock", { id })
      .then(setTimeTable)
      .catch((err) => {
        console.error(err);
        alert("ロック/アンロックの操作中にエラーが発生しました");
      });
  }, [id, setTimeTable]);

  return (
    <>
      <div
        ref={setNodeRef}
        {...listeners}
        {...attributes}
        onClick={handleDoubleClick}
        style={style}
        className={styles}
        data-tooltip-id={id.toString()}
        data-tooltip-content={toolTipMessage}
      >
        {text}
      </div>
      <Tooltip id={id.toString()} style={{ zIndex: 5 }} />
    </>
  );
}