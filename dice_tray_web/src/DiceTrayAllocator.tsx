import { useState, useEffect, useRef, useCallback } from 'react';
import {DieState, DieReaderState, DiceRequest, DieSelection} from './DataTypes' 
import { DiceBag } from './DiceBag';
import { DiceAllocatorHandle } from '../pkg/dice_wasm';
import { genSeed } from './Utility';

interface DiceTrayApplicationProps{
    appHandle: DiceAllocatorHandle
}

export function DiceTrayAllocator(props: DiceTrayApplicationProps){
    ///Set dice state.
    const [diceState, setDiceState] = useState<DieState[]>([]);

    const triggerBagRoll = useCallback(() => {
        diceState.forEach((die) => {
            if(diceSelection[die.id].isSelected){
                console.log("Triggering roll for die with ID = " + die.id);
                props.appHandle.roll_die(die.id);
            }
        })
        const diceList = props.appHandle.get_dice_state("faces").dice as DieState[];
        setDiceState(diceList);
    }, [])

    //Dice sorting
    /*
    const sortByFace = () => {
        const diceList = appHandle.get_dice_state("face").dice as DieState[];
        setDiceList(diceList);
        setSortMode("face");
    }

    const sortByResult = () => {
        const diceList = appHandle.get_dice_state("result").dice as DieState[];
        setDiceList(diceList);
        setSortMode("result");
    }
    */


    ///Set selected dice and count.
    const [diceSelection, setDiceSelection] = useState<Record<number, DieSelection>>({});
    const toggleDieSelection = useCallback((dieId: number) => {       
        setDiceSelection((prevSelected) => {
            const current = prevSelected[dieId] ?? {isSelected: false, count: 0};
            return {
                ...prevSelected,
                [dieId]: {
                    ...current,
                    isSelected: !current.isSelected
                }
            }
        })
    }, [])

    const setDieCount = useCallback((dieId: number, count: number) => {
        setDiceSelection((prevSelected) => {
            const current = prevSelected[dieId] ?? {isSelected: false, count:0};
            return {
                ...prevSelected,
                [dieId]: {
                    ...current,
                    dieCount: count
                }
            }
        })
    }, [])

    useEffect(() => {
        console.log("diceState has updated.");
        setDiceSelection((prevSelection) => {
            const currentIds = new Set(diceState.map(die => die.id));
            let hasChanges = false;
            
            const newSelection: Record<number, DieSelection> = {};
            
            for(const id in prevSelection){
                const numericId = Number(id)
                if (currentIds.has(numericId)){
                    newSelection[numericId] = prevSelection[id];
                }else{
                    console.log("found changes in dice selection.")
                    hasChanges = true;
                }
            }

            for (const die of diceState) {
                if (!(die.id in newSelection)) {
                    console.log(`Found a brand new die: ID ${die.id}`);
                    newSelection[die.id] = { isSelected: false, dieCount: 0 };
                    hasChanges = true;
                }
            }

            if(!hasChanges && Object.keys(newSelection).length === Object.keys(prevSelection).length){
                console.log("old selection = " + prevSelection);
                return prevSelection;
            }

            console.log("new dice selection = " + newSelection);
            return newSelection;
        });
    }, [diceState])

     //initialization
    const [isLoaded, setIsLoaded] = useState(false);
    const firstInit = useRef(false);

    useEffect (() => {
        //clause to prevent double fire.
        if (firstInit.current) return;
        firstInit.current = true;

        const addDice = async () => {
            try{
                //Generate a set of dice to play with.
                props.appHandle.create_die(4, genSeed());
                props.appHandle.create_die(6, genSeed());
                props.appHandle.create_die(8, genSeed());
                props.appHandle.create_die(10, genSeed());
                props.appHandle.create_die(12, genSeed());
                props.appHandle.create_die(20, genSeed());
                props.appHandle.create_die(100, genSeed());

                let diceList = props.appHandle.get_dice_state("face").dice as DieState[];                
                setDiceState(diceList);
            }catch(error){
                console.error("Caught error while creating dice: ", error);
            }finally{
                setIsLoaded(true);
            }
        };

        addDice();
    }, []);

    // mount and return the app components.
    return (
        <div className='board'>
            <DiceBag 
                diceState={diceState} 
                isLoaded={isLoaded}
                diceSelection={diceSelection} 
                toggleDieSelection={toggleDieSelection}
                triggerBagRoll={triggerBagRoll}
            />
            <div className='tray-board'>
                <div className='tray'>

                </div>
            </div>
        </div>
    )
}

