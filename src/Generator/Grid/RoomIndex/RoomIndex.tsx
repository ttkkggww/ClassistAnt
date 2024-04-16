interface roomIndexProps {
    id:number,
    name:string,
    styles:string
}

export function RoomIndex({id,name,styles}:roomIndexProps) {
    id = id + 2;
    const style = {
        gridArea: `1/${id}/2/${id+1}`,
        backgroundColor: 'white',
        border: '1px solid black',
        heignt:'100%',
        padding: '0',
    }
    const style2  = {
        lineHeight: '30px',
    }
    
    return (
        <div className={styles}style={style}>
            <div style={style2}>
                {name}
            </div>
        </div>
    )
}