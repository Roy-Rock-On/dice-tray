import { useState, useEffect, useCallback } from 'react'
import { TrayData, DieReaderData, DieReaderDetails } from "./TrayDataTypes";
import { DieReader } from "./DieReader";
import { AnimatePresence, motion } from "motion/react";

interface TrayProps {
    trayData: TrayData,
    rollTray: (trayId: string) => void;
    removeFromTray: (trayId : string) => void;
    toggleTraySelection: (trayId: string) => void;
    toggleReaderSelection: (trayId: string, readerId: number) => void;
    onTrayRollComplete: (trayId: string, readerId: number) => void;
}

const trayVariants = {
    selected: {
        outline: "4px solid #ffffff",
    },
    unselected: {
        outline: "1px solid #000000",
    }
}

export function DiceTray(props: TrayProps){
    //Add the tray ID and pass the data up.
    const toggleDieReaderSelection = useCallback((readerId: number) => {
        props.toggleReaderSelection(props.trayData.trayId, readerId);
    }, [props.trayData, props.toggleReaderSelection, props.toggleTraySelection])

    const onReaderRollComplete = useCallback((readerId: number) => {
        props.onTrayRollComplete(props.trayData.trayId, readerId);
    }, [props.trayData, props.onTrayRollComplete])

    const selectTray = () => {
        props.toggleTraySelection(props.trayData.trayId);
    }

    const triggerTrayRoll = () => {
        props.rollTray(props.trayData.trayId);
    }

    const triggerTrayRemoval = () => {
        console.log("readers slated for removal.")
        props.removeFromTray(props.trayData.trayId);
    }

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
                    {props.trayData.readerData.map((readerProp) => (
                        <motion.div
                            key={readerProp.readerDetails.reader_id}
                            layout
                            exit={{opacity:0, scale: 0.9}}
                            transition={{ type: "spring", stiffness: 500, damping: 30 }}
                        >
                            <DieReader
                                readerData={readerProp}
                                toggleSelection={toggleDieReaderSelection}
                                onRollComplete={onReaderRollComplete}
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