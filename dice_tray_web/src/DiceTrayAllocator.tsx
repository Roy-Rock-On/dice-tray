import { useState, useEffect, useRef, useCallback } from 'react';
import {DieProps, DieDetails, DieReaderState, DiceRequest} from './DataTypes' 
import { DiceBag } from './DiceBag';
import { DiceAllocatorHandle } from '../pkg/dice_wasm';
import { genSeed } from './Utility';

interface DiceTrayApplicationProps{
    appHandle: DiceAllocatorHandle
}

export function DiceTrayAllocator(props: DiceTrayApplicationProps){
    ///Set dice state.
    const [diceProps, setDiceProps] = useState<DieProps[]>([]);

    ///Update dice details from WASM
    const updateDiceProps = (diceList : DieDetails[]) => {
        setDiceProps((prevProps) => {
            const dieLookup = new Map<number, DieDetails>();
            diceList.forEach((detail) => {
                dieLookup.set(detail.id, detail);
            });

            const filteredProps =  prevProps.flatMap(prev => {
                const newDetails = dieLookup.get(prev.id);
                if (newDetails){
                    dieLookup.delete(prev.id);
                    return {
                        ...prev,
                        dieDetails: newDetails
                    }
                }
                else {
                    return [];
                }
            })

            const newDiceProps: DieProps[] = Array.from(dieLookup.values()).map(newDetails => ({
                id: newDetails.id,
                isSelected: false,
                dieCount: 0,
                dieDetails: newDetails
            }));

            return [...filteredProps, ...newDiceProps]
        })
    }

    const triggerBagRoll = useCallback(() => {
        diceProps.forEach((die) => {
            if(die.isSelected){
                console.log("Triggering roll for die with ID = " + die.id + " current face = " + die.dieDetails.current_face);
                let newDieDetails = props.appHandle.roll_die(die.id) as DieDetails;
                console.log("New face = " + newDieDetails.current_face)
            }
        })
        const diceList = props.appHandle.get_dice_state("faces").dice as DieDetails[];
        updateDiceProps(diceList);
    }, [diceProps, props.appHandle, updateDiceProps])

    ///Set dice isSelected value.
    const toggleDieSelection = useCallback((dieId: number) => {       
        setDiceProps((prevProps) => {
            return prevProps.map(prev => {
                if(prev.id == dieId){
                    const currentlySelected = prev.isSelected;
                    return {
                        ...prev,
                        isSelected: !currentlySelected
                    }
                }
                else{
                    return prev;
                }
            })
        })
    }, [])

    ///Set selected dice count
    const setDiceCount = useCallback((dieId: number, newCount: number) =>{
        setDiceProps((prevProps) => {
            return prevProps.map(prev => {
                if(prev.id == dieId){
                    return {
                        ...prev,
                        dieCount: newCount
                    }
                }
                else {
                    return prev;
                }
            })
        })
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

                let diceList = props.appHandle.get_dice_state("face").dice as DieDetails[];                
                updateDiceProps(diceList);
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
                diceProps={diceProps} 
                isLoaded={isLoaded}
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

