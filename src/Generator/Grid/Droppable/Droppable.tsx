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
  room = room + 1;
  period = period + 1;
  const style = {
    gridArea: `${room}/${period}/${room + 1}/${period+grid_size}`,
    backgroundColor: isOver ? overColor : "transparent",
  };
  return <div ref={setNodeRef} className={styles} style={style}></div>;
}
