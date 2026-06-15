import { useState, useEffect, useRef, useCallback} from "react";
import { DieReaderState, DiceRequest } from "./DataTypes";
import { DieReader } from "./DieReader";
import { AnimatePresence, motion } from "motion/react";

interface trayProps{
    trayId: string;
    rollRequest: DiceRequest[];
}

export function DiceTray(props: trayProps){
    /*
    const hasInit = useRef<boolean>(false);
    const [diceReaders, setDiceReaders] = useState<DieReaderState[]>([]);
    const [rollCount, setRollCount] = useState<number>(0);

    //Initialization.
    useEffect(() => {
        if (hasInit.current) return;
        try{
            console.log("initializing dice tray: " + props.trayId)
            //appHandle.new_tray(props.trayId);
        }
        catch{
            console.log("Failed to properly initialize dice tray with ID = " + props.trayId)
        }
        finally{
            //just for now as a test.
            //appHandle.roll_to_tray(props.trayId, 0, 1);
            //appHandle.roll_to_tray(props.trayId, 1, 8);
            //const traySummary = appHandle.roll_tray(props.trayId);
            //const dice = traySummary.tray_dice as DieReaderState[];
            setDiceReaders((prevDice) =>{
                return [...prevDice, ...dice]
            })
            hasInit.current = true;
        }
    },[])

    const [selectedDieIds, setSelectedDieIds] = useState<number[]>([]); 
    
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

    const rollDieReaders = () => {
        appHandle.roll_in_tray(props.trayId, new Uint32Array(selectedDieIds), "result");
        const traySummary = appHandle.get_tray_data(props.trayId, "result");
        const trayDice = traySummary.tray_dice as DieReaderState[];
        setDiceReaders(trayDice);
        setRollCount((lastCount) => {
            console.log("Roll count = " + lastCount);
            return lastCount += 1;
        })
    }

    const clearDieReaders = () => {
        appHandle.clear_tray_readers(new Uint32Array(selectedDieIds), props.trayId)
        const traySummary = appHandle.get_tray_data(props.trayId, "result");
        const trayDice = traySummary.tray_dice as DieReaderState[];
        setDiceReaders(trayDice);
    }

    useEffect(() => {
        console.log("received a roll request on tray = " + props.trayId);
    }, props.rollRequest)

    
    return (
        <div className="tray-group">
          <motion.div className="tray">
            <AnimatePresence mode="popLayout">
                {diceReaders.map((die_summary)=>(
                    <motion.div 
                        key={die_summary.reader_id}
                        layout
                        exit={{opacity: 0, scale: 0.9}}
                        transition={{ type: "spring", stiffness: 500, damping: 30 }}               
                    >
                        <DieReader dieReaderState={{...die_summary}} rollCount={rollCount} selectDie={selectDie} />
                    </motion.div>
                ))}
            </AnimatePresence>
          </motion.div>
          <div className="tray-tools">
            <button 
                className="button-prime"
                onClick={rollDieReaders}
            >
                Roll!
            </button>
            <button 
                className="button-destructive"
                onClick={clearDieReaders}
            >
                Clear!
            </button>
          </div>
        </div>
    )
        */
}