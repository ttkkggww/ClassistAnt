interface roomIndexProps {
    id:number,
    name:string,
    styles:string
}

export function RoomIndex({id,name,styles}:roomIndexProps) {
    id = id + 2;
    const style = {
        gridArea: `1/${id}/2/${id+1}`,
    }
    
    return (
        <div className={styles}style={style}>
            {name}
        </div>
    )
}