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
            ///Get a list of previous ID to tag dice as rolling.
            const prevReaderIds: number [] = [];
            const selectedTray = trayList?.find(tray => tray.trayId === trayId);
            selectedTray?.readerData.forEach((data) => {
                prevReaderIds.push(data.readerDetails.reader_id);
            });

            rollRequest.forEach((req) =>{
                const newReaderDetails = props.appHandle.roll_to_tray(trayId, req.dieId, req.dieCount).tray_dice as DieReaderDetails[];
                const newReaderIds: number[] = newReaderDetails.filter(detail => !prevReaderIds.includes(detail.reader_id)).map(detail => detail.reader_id);
                setTrayList(prevTrayList => {
                    return prevTrayList?.map(tray => {
                        if (tray.trayId === trayId){
                            const newData = spreadTrayDetails(tray, newReaderDetails, newReaderIds);
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

    const rollTray = useCallback((trayId: String) => {
        setTrayList(prevTrayList => {
            const selectedTrayData : TrayData | undefined = prevTrayList?.find(tray => tray.trayId === trayId);
            if (!selectedTrayData){
                console.error("No tray list available.");
                throw new Error("Tray selection failed.");
            }

            const trayLabel = selectedTrayData.trayId;
            const readerIds = selectedTrayData.readerData
                .filter(readerData => readerData.isSelected) 
                .map(readerData => readerData.readerDetails.reader_id);
            
            const newTrayDetails = props.appHandle.roll_in_tray(trayLabel, toSafeNumberArray(readerIds), "result").tray_dice as DieReaderDetails[];
            const newTrayData = spreadTrayDetails(selectedTrayData, newTrayDetails, readerIds);

            if (!prevTrayList) return prevTrayList;

            return prevTrayList?.map(tray =>{
                return tray.trayId === selectedTrayData.trayId ? newTrayData : tray;
            });
        })

    }, [trayList, props.appHandle]);

    const removeFromTray = useCallback((trayId: string) => {
        setTrayList(prevTrayList => {
            const selectedTrayData : TrayData | undefined = prevTrayList?.find(tray => tray.trayId === trayId);
            if (!selectedTrayData){
                console.error("No tray list available during remove from tray.");
                throw new Error("Tray selection failed.");
            }

            const trayLabel = selectedTrayData.trayId;
            const readerIds = selectedTrayData.readerData
                .filter(readerData => readerData.isSelected) 
                .map(readerData => readerData.readerDetails.reader_id);
            
            const newTrayDetails = props.appHandle.clear_tray_readers(toSafeNumberArray(readerIds), trayLabel).tray_dice as DieReaderDetails[];
            const newTrayData = spreadTrayDetails(selectedTrayData, newTrayDetails, []);
            if (!prevTrayList) return prevTrayList;

            return prevTrayList?.map(tray =>{
                return tray.trayId === selectedTrayData.trayId ? newTrayData : tray;
            });
        })
    }, [trayList, props.appHandle])

    const trayRollComplete = useCallback((trayId: string, readerId: number) => {
        setTrayList(prevTrayList => {
            const targetTray = prevTrayList?.find(tray => tray.trayId === trayId);
            if (!targetTray){
                console.error("No tray list available during trayRollComplete.");
                throw new Error("Tray selection failed.");
            }

            return prevTrayList?.map(tray => {
                if (tray.trayId === trayId){
                    const newReaderData = tray.readerData.map((data) => {
                        if(data.readerDetails.reader_id === readerId){
                            return {
                                ...data,
                                action: DiceAction.None
                            }
                        }
                        else{
                            return data;
                        }
                    })
                    return {
                        ...tray,
                        readerData: newReaderData
                    }
                }
                else{
                    return tray;
                }
            });
        })
    }, [trayList, props.appHandle])

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
                             trayData={tray}
                             rollTray={rollTray}
                             removeFromTray={removeFromTray}
                             toggleTraySelection={toggleTraySelection}
                             toggleReaderSelection={toggleReaderSelection}
                             onTrayRollComplete={trayRollComplete}
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

