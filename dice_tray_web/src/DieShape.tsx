interface DieShapeProps{
    dieFaces: Number
    dieColor : React.CSSProperties['color'];
}
// Combine points for the polygon attribute
const trianglePoints = "50,0 100,86.6 0,86.6";
const hexagonPoints = "25,5 75,5 100,50 75,95 25,95 0,50";
const pentagonPoints = "50,0 100,36.3 80.9,95.1 19.1,95.1 0,36.3"
const diamondPoints = "50,0 100,50 50,100 0,50"
const kitePoints = "50,0 100,70.7 50,100 0,70.7"

export function DieShape(props: DieShapeProps){
    switch (props.dieFaces){
        case 4:
            return (
                <polygon 
                    points={trianglePoints}
                    fill={props.dieColor}
                />
            )
        case 6:
            return (
                <rect
                    cx="50"
                    cy="50"
                    width={100}
                    height={100}
                    fill={props.dieColor}
                />
            )
        case 8:
            return (
                <polygon
                    points={diamondPoints}
                    fill={props.dieColor}
                />
            )
        case 10:
            return (
                <polygon
                    points={kitePoints}
                    fill={props.dieColor}
                />
            )
        case 12:
            return(
                <polygon
                    points={pentagonPoints}
                    fill={props.dieColor}
                />
            )
        case 20:
            return(
                <polygon
                    points={hexagonPoints}
                    fill={props.dieColor}
                />
            )
        default: 
            return (
                <circle
                   cx="50" 
                   cy="50"
                   r="50"
                   fill={props.dieColor} 
                />
            )
    }
}