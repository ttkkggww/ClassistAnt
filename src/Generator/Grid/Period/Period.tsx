interface periodProps {
    id:number,
    name:string,
    styles:string
}

export function Period({id,name,styles}:periodProps) {
    id = id + 2;
    const style = {
        gridArea: `${id}/1/${id+1}/2`,
    }
    
    return (
        <div className={styles}style={style}>
            {name}
        </div>
    )
}