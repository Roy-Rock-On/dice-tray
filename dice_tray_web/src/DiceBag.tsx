import { useState, useEffect, useRef, useCallback, memo } from 'react';
import { useDiceTray } from './DiceTrayContext'
import { DieView } from './DieView'
import { genSeed } from './Utility';
import { DieProps } from './DataTypes';

interface DiceList{
    dice : DieProps[];
}

export function DiceBag() {
    const appHandle = useDiceTray();

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

    const hasInit = useRef(false);
    const [diceList, setDiceList] = useState<DiceList>({ dice: [] });
    const [isLoading, setIsLoading] = useState(true);
    const [rollCount, setRollCount] = useState<number>(0);

    const triggerRoll = () => {
        setRollCount((prevCount) =>{
            return prevCount += 1;
        })
        console.log("Roll count = " + rollCount);
    }

    useEffect (() => {
        //clause to prevent double fire.
        if (hasInit.current) return;
        hasInit.current = true;

        const addDice = async () => {
            try{
                //Generate a set of dice to play with.
                appHandle.create_die(4, genSeed());
                appHandle.create_die(6, genSeed());
                appHandle.create_die(8, genSeed());
                appHandle.create_die(10, genSeed());
                appHandle.create_die(12, genSeed());
                appHandle.create_die(20, genSeed());
                appHandle.create_die(100, genSeed());

                let diceList = appHandle.get_dice_state() as DiceList;
                console.log("dice data = " + diceList);
                
                setDiceList(diceList);

            }catch(error){
                console.error("Caught error while creating dice: ", error);
            }finally{
                setIsLoading(false);
            }
        };

        addDice();
    });
    
    if (isLoading){
        return (
            <div className="dice-bag">
                <h1>Loading Dice...</h1>
            </div>
        )  
    }
    else{
        return (
            <div className="dice-bag">
                {diceList.dice.map((die_summary)=>(
                    <div key={die_summary.id}>
                        <DieView dieProps={{...die_summary}} rollCount={rollCount} selectDie={selectDie} />
                    </div>
                ))}
                <button
                    className='button-prime'
                    onClick={triggerRoll}
                >
                    Roll!
                </button>
            </div> 
        )

    }
}