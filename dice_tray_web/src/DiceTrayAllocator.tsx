import { useState, useEffect, useRef, useCallback } from 'react';

import { 
    DieData,
    DieDetails,
    NewDieRequest,
    ReaderRequest,
    spreadDieDetails,
    getReaderRequest
} from './DieDataTypes'

import {
    TrayData,
    DieReaderDetails,
    NewTrayRequest,
    spreadTrayDetails,
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
    ///Set dice state.
    const [diceData, setDiceData] = useState<DieData[]>([]);

    ///Update dice details from WASM
    const updateDiceData = (diceDetails : DieDetails[]) => {
        setDiceData((prevData) => {
           return spreadDieDetails(prevData, diceDetails)
        })
    }

    const triggerBagRoll = useCallback(() => {
        diceData.forEach((die) => {
            if(die.isSelected){
                console.log("Triggering roll for die with ID = " + die.id + " current face = " + die.dieDetails.current_face);
                let newDieDetails = props.appHandle.roll_die(die.id) as DieDetails;
                console.log("New face = " + newDieDetails.current_face)
            }
        })
        const diceList = props.appHandle.get_dice_state("faces").dice as DieDetails[];
        updateDiceData(diceList);
    }, [diceData, props.appHandle, updateDiceData])

    
    const destroyDice = useCallback(() => {
        const selectedDieIds: number[] = diceData
            .filter(die => die.isSelected)
            .map(die => die.id);
        try{
            const safeIds = toSafeNumberArray(selectedDieIds);
            const newDiceDetails = props.appHandle.destroy_dice(safeIds).dice as DieDetails[];
            updateDiceData(newDiceDetails);
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

    const [trayList, setTrayList] = useState<TrayData[]>();
    const toggleTraySelection = useCallback((trayId: string) =>{
        const rollRequest = getReaderRequest(diceData);
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
            rollRequest.forEach((req) =>{
                const newReaderDetails = props.appHandle.roll_to_tray(trayId, req.dieId, req.dieCount).tray_dice as DieReaderDetails[];
                setTrayList(prevTrayList => {
                    return prevTrayList?.map(tray => {
                        if (tray.trayId === trayId){
                            const newData = spreadTrayDetails(tray, newReaderDetails);
                            newData.isSelected = true;
                            return newData;
                        }
                        else{
                            return{
                                ...tray,
                                isSelected: false
                            }
                        }
                    })
                })
            });
            clearDieSelection();
        }

    }, [trayList, diceData, props.appHandle] )

    const toggleReaderSelection = useCallback((trayId: string, readerId: number) =>{
        setTrayList(prevTrayList => {
            if (!prevTrayList) { return prevTrayList; }

            return prevTrayList.map(tray => {
                if(tray.trayId === trayId){
                    return {
                        ...tray,
                        readerData: tray.readerData.map((reader) => {
                            if (reader.readerDetails.reader_id === readerId) {
                                return {
                                    ...reader,
                                    isSelected: !reader.isSelected
                                };
                            }
                            return { ...reader }; // Return unchanged reader clone
                        })
                    };
                }
                else{
                    return {
                        ...tray,
                        readerData: tray.readerData.map((reader) => ({
                            ...reader,
                            isSelected: false
                        }))
                    };
                }
            })
        })
    }, [trayList, props.appHandle])

    const rollTray = useCallback(() => {
        const selectedTrayData : TrayData | undefined = trayList?.find(tray => tray.isSelected) as TrayData;
        if (!selectedTrayData){
            console.error("No tray list available.");
            throw new Error("Tray selection failed.");
        }

        const trayLabel = selectedTrayData.trayId;
        const readerIds = selectedTrayData.readerData
            .filter(readerData => readerData.isSelected) 
            .map(readerData => readerData.readerDetails.reader_id);
        
        const newTrayDetails = props.appHandle.roll_in_tray(trayLabel, toSafeNumberArray(readerIds), "result").tray_dice as DieReaderDetails[];
        const newTrayData = spreadTrayDetails(selectedTrayData, newTrayDetails)

        setTrayList(prevTrayList => {
            prevTrayList?.map(tray =>{
                tray.trayId === selectedTrayData.trayId ? newTrayData : tray
            });
        })

    }, [trayList, props.appHandle]);

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
            readerData: [],
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

    ///New Die Modal Form
    const [isNewDieModalOpen, setIsNewDieModalOpen] = useState(false);

    const openNewDieModal = () => {
        console.log("New die modal is opening! I hope...");
        setIsNewDieModalOpen(true);
    }

    const onSubmitNewDie = (newDieRequest: NewDieRequest) => {
        props.appHandle.create_die(newDieRequest.sides, genSeed(), newDieRequest.label, newDieRequest.variance);
        const newDieDetails = props.appHandle.get_dice_state("face").dice as DieDetails[];
        updateDiceData(newDieDetails);
        setIsNewDieModalOpen(false);
    }

    const onCloseNewDieFrom = () => {
        setIsNewDieModalOpen(false);
        console.log("New die form has been closed.");
    }

    //initialization
    const [isLoaded, setIsLoaded] = useState(false);
    const firstInit = useRef(false);

    useEffect (() => {
        //clause to prevent double fire.
        if (firstInit.current) return;
        firstInit.current = true;

        const addDice = async () => {
            try{
                let diceList = props.appHandle.get_dice_state("face").dice as DieDetails[];                
                updateDiceData(diceList);
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
                diceData={diceData} 
                isLoaded={isLoaded}
                toggleDieSelection={toggleDieSelection}
                triggerBagRoll={triggerBagRoll}
                setDieCount={setDieCount}
                openNewDieModal={openNewDieModal}
                destroyDice={destroyDice}
            />
            <div className='tray-board'>
                {trayList?.map((tray) => (
                    <div 
                        key={tray.trayId}
                    >
                        <DiceTray
                             trayData={tray}
                             rollTray={rollTray}
                             toggleTraySelection={toggleTraySelection}
                             toggleReaderSelection={toggleReaderSelection}
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
}

