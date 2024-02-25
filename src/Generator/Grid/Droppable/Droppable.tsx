import { useDroppable } from "@dnd-kit/core";

interface droppableProps {
  id: number;
  styles: string;
}

export function Droppable({ id, styles }: droppableProps) {
  const { isOver, setNodeRef } = useDroppable({
    id: id.toString(),
  });

  const style = {
    color: isOver ? "green" : undefined,
  };
  return <div ref={setNodeRef} className={styles} style={style}></div>;
}
