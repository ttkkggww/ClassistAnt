
import {useDraggable} from "@dnd-kit/core";

interface DraggableProps {
    hex_color: string;
    text: string;
    id: number;
    styles: string;
    classId : number;
}

export function Draggable({hex_color, text,id,styles}: DraggableProps)  {
    const {attributes, listeners, setNodeRef, transform} = useDraggable({
        id:id.toString(),
    });
    const style = transform ? {
        transform: `translate3d(${transform.x}px, ${transform.y}px, 0)`,
    } : undefined;

    return (
        <div ref={setNodeRef} style={style} {...listeners} {...attributes} >
            <div style={{backgroundColor:hex_color}} className={styles}>
                {text}
            </div>
        </div>
    );
}