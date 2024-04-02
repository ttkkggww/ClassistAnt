import { useDroppable } from "@dnd-kit/core";

interface droppableProps {
  id: number;
  styles: string;
  grid_size: number;
}

export function Droppable({ id,styles,grid_size }: droppableProps) {
  const { isOver, setNodeRef } = useDroppable({
    id: id.toString(),
  });

  const style = {
    gridColumn: `span ${grid_size}`,
    backgroundColor: isOver ? "rgba(0, 0, 0, 0.1)" : "transparent",
  };
  return <div ref={setNodeRef} className={styles} style={style}></div>;
}
