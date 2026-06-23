import { useState, useEffect, useRef, useCallback } from 'react';

import { 
    DieData,
    DieDetails,
    NewDieRequest,
    ReaderRequest,
    spreadDieDetails,
    getReaderRequest,
    DiceAction
} from './DieDataTypes'

import {
    TrayData,
    NewTrayRequest,
} from './TrayDataTypes'
    
import { DiceBag } from './DiceBag';
import { DiceAllocatorHandle } from '../pkg/dice_wasm';
import { genSeed, toSafeNumberArray } from './Utility';
import { NewDieModal } from './NewDieFrom';
import { NewTrayModal } from './NewTrayForm';
import { DiceTray } from './DiceTray';

interface DiceTrayApplicationProps{
    appHandle: DiceAllocatorHandle
}

export function DiceTrayAllocator(props: DiceTrayApplicationProps){
    //#region DICE BAG
    ///Set dice state.
    const [diceData, setDiceData] = useState<DieData[]>([]);

    ///Update dice details from WASM
    const updateDiceData = (diceDetails : DieDetails[], rolledDice: number[]) => {
        setDiceData((prevData) => {
           return spreadDieDetails(prevData, diceDetails, rolledDice)
        })
    }

    const triggerBagRoll = useCallback(() => {
        const rolledList: number[] = [];
        diceData.forEach((die) => {
            if(die.isSelected){
                console.log("Triggering roll for die with ID = " + die.id + " current face = " + die.dieDetails.current_face);
                rolledList.push(die.id);
                let newDieDetails = props.appHandle.roll_die(die.id) as DieDetails;
                console.log("New face = " + newDieDetails.current_face)
            }
        })
        const diceList = props.appHandle.get_dice_state("faces").dice as DieDetails[];
        updateDiceData(diceList, rolledList);
    }, [diceData, props.appHandle, updateDiceData])

    
    const destroyDice = useCallback(() => {
        const selectedDieIds: number[] = diceData
            .filter(die => die.isSelected)
            .map(die => die.id);
        try{
            const safeIds = toSafeNumberArray(selectedDieIds);
            const newDiceDetails = props.appHandle.destroy_dice(safeIds).dice as DieDetails[];
            updateDiceData(newDiceDetails, []);
        }
        catch{
            console.error("Could not cast IDs safely while attempting to Destroy Dice.");
        }
    }, [diceData, props.appHandle, updateDiceData])

    ///Set dice isSelected value.
    const toggleDieSelection = useCallback((dieId: number) => {       
        setDiceData((prevProps) => {
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
    }, [diceData])

    const clearDieSelection = () => {
        console.log("Clearing dice selection now.");
        setDiceData((prevProps) => {
            return prevProps.map(prev => {
                return {
                    ...prev,
                    isSelected: false
                }
            })
        })
    }

    ///Set selected dice count
    const setDieCount = useCallback((dieId: number, newCount: number) =>{
        setDiceData((prevData) => {
            return prevData.map(prev => {
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
    }, [diceData])

    const onRollComplete = useCallback((dieId: number) => {
        setDiceData((prevData) => {
            return prevData.map(prev => {
                if(prev.id === dieId){
                    return{
                        ...prev,
                        action: DiceAction.None
                    }
                }
                else{
                    return prev;
                }
            })
        })
    }, [diceData]) 

    //#endregion

    //#region TRAY LIST
    const [trayList, setTrayList] = useState<TrayData[]>();
    
    const toggleTraySelection = useCallback((trayId: string) =>{
        const rollRequest: ReaderRequest[] = getReaderRequest(diceData);
        if (!rollRequest){
            console.log("No roll requests found. Triggering tray selection toggle.")
            setTrayList(prevTrayList => {
                return prevTrayList?.map(tray => {
                    if (tray.trayId === trayId){
                        return {
                            ...tray,
                            isSelected: true
                        }
                    }
                    else{
                        return {
                            ...tray,
                            isSelected: false
                        }
                    }
                })
            })  
        }else{
            console.log("Here's where we should trigger a tray roll and update the tray.");
            setTrayList((prevList) => {
                return prevList?.map((prev) => {
                    if (prev.trayId === trayId){
                        return {
                            ...prev,
                            readerRequest: rollRequest
                        }
                    }
                    else{
                        return prev
                    }
                })
            })

            clearDieSelection();
        }

    }, [trayList, diceData, props.appHandle] )

    //#endregion

    //#region NEW TRAY MODAL
    ///New Tray Modal Form
    const [isNewTrayModalOpen, setIsNewTrayModalOpen] = useState(false);

    const openNewTrayModal = () => {
        console.log("New tray modal is opening!");
        setIsNewTrayModalOpen(true);
    }

    const onSubmitNewTray = (newTrayRequest: NewTrayRequest) => {
        const newTrayDetails = props.appHandle.new_tray(newTrayRequest.label);
        const newTrayProps: TrayData = {
            trayId: newTrayDetails.tray_label as string,
            isSelected: false,
            readerRequest: []
        };

        setTrayList((prevList) => {
            const currentList = prevList ?? [];
            return [...currentList, newTrayProps];
        })

        setIsNewTrayModalOpen(false);
    }

    const onCloseNewTrayForm = () => {
        setIsNewTrayModalOpen(false);
        console.log("New tray form has been closed.");
    }
    //#endregion

    //#region NEW DIE MODAL
    ///New Die Modal Form
    const [isNewDieModalOpen, setIsNewDieModalOpen] = useState(false);

    const openNewDieModal = () => {
        console.log("New die modal is opening! I hope...");
        setIsNewDieModalOpen(true);
    }

    const onSubmitNewDie = (newDieRequest: NewDieRequest) => {
        props.appHandle.create_die(newDieRequest.sides, genSeed(), newDieRequest.label, newDieRequest.variance);
        const newDieDetails = props.appHandle.get_dice_state("face").dice as DieDetails[];
        updateDiceData(newDieDetails, []);
        setIsNewDieModalOpen(false);
    }

    const onCloseNewDieFrom = () => {
        setIsNewDieModalOpen(false);
        console.log("New die form has been closed.");
    }
    //#endregion

    //#region INITIALIZATION
    //initialization
    const [isLoaded, setIsLoaded] = useState(false);
    const firstInit = useRef(false);

    useEffect (() => {
        if (firstInit.current) return;
        firstInit.current = true;

        const addDice = async () => {
            try{
                let diceList = props.appHandle.get_dice_state("face").dice as DieDetails[];                
                updateDiceData(diceList, []);
            }catch(error){
                console.error("Caught error while creating dice: ", error);
            }finally{
                setIsLoaded(true);
            }
        };

        addDice();
    }, []);
    //#endregion

    //#region COMPONENT RETURN
    return (
        <div className='board'>
            <DiceBag 
                diceData={diceData} 
                isLoaded={isLoaded}
                toggleDieSelection={toggleDieSelection}
                triggerBagRoll={triggerBagRoll}
                setDieCount={setDieCount}
                openNewDieModal={openNewDieModal}
                destroyDice={destroyDice}
                setDiceBag={setDiceData}
                onRollComplete={onRollComplete}
            />
            <div className='tray-board'>
                {trayList?.map((tray) => (
                    <div 
                        key={tray.trayId}
                    >
                        <DiceTray
                            appHandle={props.appHandle}
                            trayData={tray}
                            toggleTraySelection={toggleTraySelection}
                        />
                    </div>
                ))}
                <button
                    className='button-prime'
                    onClick={openNewTrayModal}
                >
                    New Tray
                </button>
            </div>
            <NewDieModal
                isOpen={isNewDieModalOpen}
                onSubmitNewDie={onSubmitNewDie}
                onClose={onCloseNewDieFrom}
            />
            <NewTrayModal
                isOpen={isNewTrayModalOpen}
                onSubmitNewTray={onSubmitNewTray}
                onClose={onCloseNewTrayForm}
            />
        </div>
    )
    //#endregion
}

