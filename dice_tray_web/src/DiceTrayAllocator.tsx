import { useState, useEffect, useRef, useCallback } from 'react';
import { 
        DieProps,
        DieDetails,
        TrayProps,
        DieReaderDetails,
        NewDieRequest,
        NewTrayRequest,
        spreadTrayDetails
    } from './DataTypes' 
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

    
    const destroyDice = useCallback(() => {
        const selectedDieIds: number[] = diceProps
            .filter(die => die.isSelected)
            .map(die => die.id);
        try{
            const safeIds = toSafeNumberArray(selectedDieIds);
            const newDiceDetails = props.appHandle.destroy_dice(safeIds).dice as DieDetails[];
            updateDiceProps(newDiceDetails);
        }
        catch{
            console.error("Could not cast IDs safely while attempting to Destroy Dice.");
        }
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
    }, [diceProps])

    ///Set selected dice count
    const setDieCount = useCallback((dieId: number, newCount: number) =>{
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
    }, [diceProps])

    const [trayList, setTrayList] = useState<TrayProps[]>();
    const newDiceTray = () => {

    }

    const toggleTraySelection = useCallback((trayId: string) =>{
        setTrayList(prevTrayList => {
            prevTrayList?.map(tray => {
                if (tray.trayId === trayId){
                    return {
                        ...tray,
                        isSelected: !tray.isSelected
                    }
                }
                else{
                    return tray;
                }
            })
        })
        
    }, [trayList, props.appHandle] )

    const rollTray = useCallback(() => {
        const selectedTrayProps : TrayProps | undefined = trayList?.find(tray => tray.isSelected) as TrayProps;
        if (!selectedTrayProps){
            console.error("No tray list available.");
            throw new Error("Tray selection failed.");
        }

        const trayLabel = selectedTrayProps.trayId;
        const readerIds = selectedTrayProps.readerProps
            .filter(readerProp => readerProp.isSelected)
            .map(readerProp => readerProp.id);
        
        const newTrayDetails = props.appHandle.roll_in_tray(trayLabel, toSafeNumberArray(readerIds), "result")
            .tray_dice as DieReaderDetails[];

        const newTrayProps = spreadTrayDetails(selectedTrayProps, newTrayDetails)

        setTrayList(prevTrayList => {
            prevTrayList?.map(tray =>{
                tray.trayId === selectedTrayProps.trayId ? newTrayProps : tray
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
        const newTrayProps: TrayProps = {
            trayId: newTrayDetails.tray_label as string,
            isSelected: false,
            readerProps: [],
            rollTray: rollTray,
            toggleSelection: toggleTraySelection
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
        updateDiceProps(newDieDetails);
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
                             trayId={tray.trayId}
                             isSelected={tray.isSelected}
                             readerProps={tray.readerProps}
                             rollTray={rollTray}
                             toggleSelection={toggleTraySelection}
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

