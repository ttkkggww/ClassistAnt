import { useDroppable } from "@dnd-kit/core";

interface droppableProps {
  id: number;
  styles: string;
  grid_size: number;
  overColor: string;
}

export function Droppable({ id,styles,grid_size ,overColor}: droppableProps) {
  const { isOver, setNodeRef } = useDroppable({
    id: id.toString(),
  });

  const style = {
    gridColumn: `span ${grid_size}`,
    backgroundColor: isOver ? overColor : "transparent",
  };
  return <div ref={setNodeRef} className={styles} style={style}></div>;
}
