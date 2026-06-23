import { useState, useEffect, useCallback, memo, useRef } from 'react'
import { TrayData, DieReaderData, DieReaderDetails, spreadReaderDetails } from "./TrayDataTypes";
import { DieReader } from "./DieReader";
import { AnimatePresence, motion } from "motion/react";
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

    return (
        <div className='tray-group'>
            <motion.div
                className='tray'
                style={{ overflow: 'visible' }}
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
                <AnimatePresence mode='popLayout'>
                    {dieReaders?.map((readerData) => (
                        <motion.div
                            key={readerData.readerDetails.reader_id}
                            layout
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