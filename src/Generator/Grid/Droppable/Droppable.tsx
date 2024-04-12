import { useDroppable } from "@dnd-kit/core";

interface droppableProps {
  id: number;
  styles: string;
  room: number;
  period: number;
  grid_size: number;
  overColor: string;
}

export function Droppable({ id,styles,room,period,grid_size ,overColor}: droppableProps) {
  const { isOver, setNodeRef } = useDroppable({
    id: id.toString(),
  });
  room = room + 2;
  period = period + 2;
  const style = {
    gridArea: `${period}/${room}/${period+grid_size}/${room + 1}`,
    backgroundColor: isOver ? overColor : "transparent",
  };
  return <div ref={setNodeRef} className={styles} style={style}></div>;
}
