import { useEffect, useState} from "react";
import { DiceAllocatorHandle } from "../pkg/dice_wasm";

export interface DieProps {
    id: number;  
    label: string;
    faces: number;
    current_face: number;
    result: string;
}

export function Die(props: DieProps) {
    const [dieProps, setDieProps] = useState<DieProps>(props);
    const rollDie = (() => {
        try{
            //let summary = appHandle.roll_die(dieProps.id);
            //setDieProps(JSON.parse(summary));
        }
        catch(error){
            console.log("Error rolling die:", error);
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