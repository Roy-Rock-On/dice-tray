import { useState, useEffect, useRef, useCallback} from "react";
import { DieProps, RollRequest } from "./DataTypes";
import { useDiceTray } from "./DiceTrayContext";
import { DieView } from "./DieView";

interface trayProps{
    trayId: string;
    rollRequest: RollRequest[];
}

export function DiceTray(props: trayProps){
    const hasInit = useRef<boolean>(false);
    const appHandle = useDiceTray();
    const [diceReaders, setDiceReaders] = useState<DieProps[]>([]);

    useEffect(() => {
        if (hasInit.current) return;
        try{
            console.log("initializing dice tray: " + props.trayId)
            appHandle.new_tray(props.trayId);
        }
        catch{
            console.log("Failed to properly initialize dice tray with ID = " + props.trayId)
        }
        finally{
            //just for now as a test.
            appHandle.roll_to_tray(props.trayId, 0, 4);
            const traySummary = appHandle.roll_tray(props.trayId);
            const dice = traySummary.tray_dice as DieProps[];
            setDiceReaders((prevDice) =>{
                return [...prevDice, ...dice]
            })
            hasInit.current = true;
        }
    },[])

    const [selectedDieIds, setSelectedDieIds] = useState<number[]>([]); 
    console.log("Currently selected = " + selectedDieIds);
    
    const selectDie = useCallback((dieID: number, isSelected: boolean) => {       
        setSelectedDieIds((prevSelected) => {
            if (isSelected){
                return [...prevSelected, dieID]
            }
            else{
                return prevSelected.filter(id => dieID !== id);
            }
        })
    }, [])

    useEffect(() => {
        console.log("received a roll request on tray = " + props.trayId);
    }, props.rollRequest)

    return (
        <div className="tray-group">
          <div className="tray">
            {diceReaders.map((die_summary)=>(
                <div key={die_summary.id}>
                    <DieView dieProps={{...die_summary}} rollCount={0} selectDie={selectDie} />
                </div>
            ))}
          </div>
          <div className="tray-tools">
            <button className="button-prime">Roll!</button>
            <button className="button-destructive">Clear!</button>
          </div>
        </div>
    )
}