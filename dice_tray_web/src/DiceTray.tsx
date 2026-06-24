import { useState, useEffect, useCallback, memo, useRef } from 'react'
import { TrayData, DieReaderData, DieReaderDetails, spreadReaderDetails } from "./TrayDataTypes";
import { DieReader } from "./DieReader";
import { AnimatePresence, motion, LayoutGroup } from "motion/react";
import { DiceAllocatorHandle } from '../pkg/dice_wasm';
import { toSafeNumberArray } from './Utility';
import { DiceAction, ReaderRequest } from './DieDataTypes';

interface TrayProps {
    trayData: TrayData,
    appHandle: DiceAllocatorHandle,
    toggleTraySelection: (trayId: string) => void,
}

const trayVariants = {
    selected: {
        outline: "4px solid #ffffff",
    },
    unselected: {
        outline: "1px solid #000000",
    }
}

export function DiceTrayComponent(props: TrayProps){
    //#region DIE READERS
    const [dieReaders, setDieReaders] = useState<DieReaderData[]>();
    const lastRollRequest = useRef<number>(0);

    const selectTray = () => {
        props.toggleTraySelection(props.trayData.trayId);
    }

    const triggerTrayRoll = () => {
        const selectedReaderIds = (dieReaders ?? []).filter(reader => reader.isSelected).map(reader => reader.readerDetails.reader_id);
        const newReaderDetails = props.appHandle.roll_in_tray(props.trayData.trayId, toSafeNumberArray(selectedReaderIds), "face").tray_dice as DieReaderDetails[];
        setDieReaders((prevReaders) => {
            return spreadReaderDetails((prevReaders ?? []), newReaderDetails, selectedReaderIds);
        });
    }

    useEffect(() => {
        if(!props.trayData.rollRequest) return;
        if(lastRollRequest.current === props.trayData.rollRequest.lastRequestId) return;
        lastRollRequest.current = props.trayData.rollRequest?.lastRequestId;

        const newReaderRequests : ReaderRequest[] = props.trayData.rollRequest.request;
        newReaderRequests.forEach((request) => {
            props.appHandle.roll_to_tray(props.trayData.trayId, request.dieId, request.dieCount);
        });

        const newReaderDetails = props.appHandle.get_tray_summary(props.trayData.trayId, "result").tray_dice as DieReaderDetails[];

        setDieReaders((prevReaders) => {
            const oldReaderIds = prevReaders?.map(reader => reader.readerDetails.reader_id); 
            const rolledReaderIds = newReaderDetails.filter(detail => !oldReaderIds?.includes(detail.reader_id)).map(detail => detail.reader_id);
            return spreadReaderDetails((prevReaders ?? []), newReaderDetails, rolledReaderIds);
        }); 
    }, [props.trayData.rollRequest])

    const triggerTrayRemoval = () => {
        const selectedReaderIds = (dieReaders ?? []).filter(reader => reader.isSelected).map(reader => reader.readerDetails.reader_id);
        const newReaderDetails = props.appHandle.clear_tray_readers(toSafeNumberArray(selectedReaderIds), props.trayData.trayId).tray_dice as DieReaderDetails[];
        setDieReaders((prevReaders) => {
            return spreadReaderDetails((prevReaders ?? []), newReaderDetails, []);
        })
    }

    const toggleReaderSelection = useCallback((readerId: number) => {
        setDieReaders((prevReaders) => {
            return prevReaders?.map((prev) =>{
                if (prev.readerDetails.reader_id === readerId){
                    return {
                        ...prev,
                        isSelected: !prev.isSelected
                    }
                }
                else{
                    return prev;
                }
            })
        })
    }, [dieReaders, props.appHandle])

    const readerRollComplete = useCallback((readerId: number) => {
        setDieReaders((prevReaders) => {
            return prevReaders?.map((prev) => {
                if (prev.readerDetails.reader_id === readerId){
                    return {
                        ...prev,
                        action: DiceAction.None
                    }
                }
                else{
                    return prev;
                }
            })
        })
    }, [dieReaders, props.appHandle])
    //#endregion

    //#region GRID TOOLS
    const handleGridReorder = (currentIndex: number, offset: { x: number; y: number }) => {
    // Width (60px) + Gap (12px) = 72px total slot size
    const SLOT_SIZE = 72; 
    
    // 💡 UPDATE THIS: How many items fit horizontally in your .tray CSS grid?
    const COLS_PER_ROW = 6; 

    const colOffset = Math.round(offset.x / SLOT_SIZE);
    const rowOffset = Math.round(offset.y / SLOT_SIZE);
    
    let targetIndex = currentIndex + colOffset + (rowOffset * COLS_PER_ROW);
    targetIndex = Math.max(0, Math.min(targetIndex, (dieReaders?.length ?? 1) - 1));
    
    if (targetIndex !== currentIndex && dieReaders) {
        const updatedList = [...dieReaders];
        const [movedItem] = updatedList.splice(currentIndex, 1);
        updatedList.splice(targetIndex, 0, movedItem);
        setDieReaders(updatedList);
    }
};

    return (
        <div className='tray-group'>
            <LayoutGroup>
                <motion.div
                    className='tray'
                    style={{ 
                        overflow: 'visible',
                        display: 'grid',
                        gridTemplateColumns: 'repeat(auto-fill, minmax(60px, 1fr))',
                        gap: '12px'
                    }}
                    animate={props.trayData.isSelected ? "selected" : "unselected"}
                    variants={trayVariants}
                    whileHover={{
                        scale: 1.02,
                        boxShadow: '0px 10px 30px rgba(244, 242, 247, 0.3)'
                    }}
                    transition={{
                        type: 'spring',
                        stiffness: 300,
                        damping: 20
                    }}
                    role="button"
                    tabIndex={0}
                    onClick={selectTray}
                >
                    <AnimatePresence mode='sync'>
                        {dieReaders?.map((readerData, index) => (
                            <motion.div
                                key={readerData.readerDetails.reader_id}
                                layout
                                
                                drag
                                dragConstraints={{top: 0, left: 0, right: 0, bottom: 0}}
                                dragElastic={1}

                                whileDrag={{zIndex: 10, scale: 1.1}}
                                onDragEnd={(_, info) => handleGridReorder(index, info.offset)}

                                exit={{opacity:0, scale: 0.9}}
                                transition={{ type: "spring", stiffness: 500, damping: 30 }}
                            >
                                <DieReader
                                    readerData={readerData}
                                    toggleSelection={toggleReaderSelection}
                                    onRollComplete={readerRollComplete}
                                />
                            </motion.div>
                        ))}
                    </AnimatePresence>
                </motion.div>
            </LayoutGroup>
            <div className='tray-tools'>
                <button 
                    className='button-prime'
                    onClick={triggerTrayRoll}
                >
                    Roll
                </button>
                <button 
                    className='button-destructive'
                    onClick={triggerTrayRemoval}
                >
                    Remove
                </button>
            </div>
        </div>
    )
}

export const DiceTray = memo(DiceTrayComponent);   