import { useState, useEffect, useRef, useCallback } from 'react';
import { useDiceTray } from './DiceTrayContext'
import { DieView } from './DieView'
import { genSeed } from './Utility';
import { DieProps } from './DataTypes';
import { motion, AnimatePresence, MotionConfig } from 'motion/react';

export function DiceBag() {
    const appHandle = useDiceTray();
    const [diceList, setDiceList] = useState<DieProps[]>([]);
    const [selectedDieIds, setSelectedDieIds] = useState<number[]>([]); 
    const [rollCount, setRollCount] = useState<number>(0);
    const [sortMode, setSortMode] = useState<string>("face");

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

    const hasInit = useRef(false);
    const [isLoading, setIsLoading] = useState(true);

    const triggerRoll = () => {
        selectedDieIds.forEach((x) => {
            appHandle.roll_die(x);
        })
        const diceList = appHandle.get_dice_state(sortMode).dice as DieProps[];
        setDiceList(diceList);
        setRollCount((lastCount) => {
            console.log("Roll count = " + lastCount);
            return lastCount += 1;
        })
    }

    const sortByFace = () => {
        const diceList = appHandle.get_dice_state("face").dice as DieProps[];
        setDiceList(diceList);
        setSortMode("face");
    }

    const sortByResult = () => {
        const diceList = appHandle.get_dice_state("result").dice as DieProps[];
        setDiceList(diceList);
        setSortMode("result");
    }
    

    useEffect (() => {
        //clause to prevent double fire.
        if (hasInit.current) return;
        hasInit.current = true;

        const addDice = async () => {MotionConfig
            try{
                //Generate a set of dice to play with.
                appHandle.create_die(4, genSeed());
                appHandle.create_die(6, genSeed());
                appHandle.create_die(8, genSeed());
                appHandle.create_die(10, genSeed());
                appHandle.create_die(12, genSeed());
                appHandle.create_die(20, genSeed());
                appHandle.create_die(100, genSeed());

                let diceList = appHandle.get_dice_state("face").dice as DieProps[];                
                setDiceList(diceList);
            }catch(error){
                console.error("Caught error while creating dice: ", error);
            }finally{
                setIsLoading(false);
            }
        };

        addDice();
    }, []);
    
    if (isLoading){
        return (
            <div className="dice-bag">
                <h1>Loading Dice...</h1>
            </div>
        )  
    }
    else{
        return (
            <motion.div className="dice-bag">
                <AnimatePresence mode="popLayout">
                    {diceList.map((die_state)=>(
                        <motion.div 
                            key={die_state.id}
                            layout
                            exit={{opacity: 0, scale: 0.9}}
                            transition={{ type: "spring", stiffness: 500, damping: 30 }}
                        >
                            <DieView dieState={{...die_state}} rollCount={rollCount} selectDie={selectDie} />
                        </motion.div>
                    ))}
                </AnimatePresence>
                <button
                    className='button-prime'
                    onClick={triggerRoll}
                >
                    Roll!
                </button>
                <button
                    className='button-prime'
                    onClick={sortByFace}
                >
                    Face
                </button>
                <button
                    className='button-prime'
                    onClick={sortByResult}
                >
                    Result
                </button>
            </motion.div> 
        )

    }
}