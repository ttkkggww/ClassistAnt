import { useDraggable } from "@dnd-kit/core";
import {invoke} from "@tauri-apps/api/tauri";
import { TimeTable } from "../Grid";

interface DraggableProps {
  hex_color: string;
  text: string;
  id: number;
  styles: string;
  classId: number;
  grid_size: number;
  setTimeTable: (
    timeTable: TimeTable | ((prevTimeTable: TimeTable) => TimeTable),
  ) => void;
}

export function Draggable({ hex_color, text, id, styles,grid_size,setTimeTable }: DraggableProps) {
  const { attributes, listeners, setNodeRef, transform } = useDraggable({
    id: id.toString(),
  });
  const style = transform
    ? {
        transform: `translate3d(${transform.x}px, ${transform.y}px, 0)`,
        backgroundColor: hex_color,
        gridColumn: `span ${grid_size}`,
      }
    : {
        backgroundColor: hex_color,
        gridColumn: `span ${grid_size}`,
    };

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
      {...listeners} 
      {...attributes}
      onDoubleClick={handleDobuleClick}
      style={style} className={styles}>

        {text}
    </div>
  );
}
