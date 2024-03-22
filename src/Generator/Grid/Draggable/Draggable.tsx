import { useDraggable } from "@dnd-kit/core";
import {invoke} from "@tauri-apps/api/tauri";
import { TimeTable } from "../Grid";

interface DraggableProps {
  hex_color: string;
  text: string;
  id: number;
  styles: string;
  classId: number;
  setTimeTable: (
    timeTable: TimeTable | ((prevTimeTable: TimeTable) => TimeTable),
  ) => void;
}

export function Draggable({ hex_color, text, id, styles,classId,setTimeTable }: DraggableProps) {
  const { attributes, listeners, setNodeRef, transform } = useDraggable({
    id: id.toString(),
  });
  const style = transform
    ? {
        transform: `translate3d(${transform.x}px, ${transform.y}px, 0)`,
      }
    : undefined;

  const handleDobuleClick = () => {
    invoke<TimeTable>("handle_switch_lock", {id:id})
    .then((res) => {
      setTimeTable(res);
    }).catch((err) => {
      console.log(err);
    });
  }
  return (
    <div ref={setNodeRef} 
      style={style} 
      {...listeners} 
      {...attributes}
      onDoubleClick={handleDobuleClick}
    >
      <div style={{ backgroundColor: hex_color }} className={styles}>
        {text}
      </div>
    </div>
  );
}
