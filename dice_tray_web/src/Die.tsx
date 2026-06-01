import { useEffect, useState} from "react";
import { useDiceTray } from "./DiceTrayContext";


export interface DieProps {
    id: number;  
    label: string;
    faces: number;
    current_face: number;
    result: string;
}

export function Die(props: DieProps) {
    const appHandle = useDiceTray();
    const [dieProps, setDieProps] = useState<DieProps>(props);

    let isRolling = false;
    const rollDie = (() => {
        if(isRolling) return;
        isRolling = true;
        try{
            let summary = appHandle.roll_die(dieProps.id) as DieProps;
            setDieProps(summary);
        }
        catch(error){
            console.log("Error rolling die:", error);
        }finally{
            isRolling = false;
        }
    });

    return (
        <button 
            className="die"
            onClick={rollDie}>
            <p>{dieProps.current_face}</p>
        </button>
    );
}