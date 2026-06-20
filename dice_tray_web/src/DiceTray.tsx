import { useState, useEffect, useCallback } from 'react'
import { DieReaderProps, DieReaderDetails, TrayProps } from "./DataTypes";
import { DieReader } from "./DieReader";
import { AnimatePresence, motion } from "motion/react";
import { DiceAllocatorHandle } from '../pkg/dice_wasm';

const trayVariants = {
    selected: {
        scale: 1.20,
        stroke: '#ffffff'
    },
    unselected: {
        scale: 1,
        stroke: '#000000'
    }
}


export function DiceTray(props: TrayProps){

    const selectTray = () => {
        props.toggleSelection(props.trayId)
    }

    return (
        <div className='tray-group'>
            <motion.div
                className='tray'
                animate={props.isSelected ? "selected" : "unselected"}
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
                    {props.readerProps.map((readerProp) => (
                        <motion.div
                            key={readerProp.id}
                            layout
                            exit={{opacity:0, scale: 0.9}}
                            transition={{ type: "spring", stiffness: 500, damping: 30 }}
                        >
                            <DieReader
                                readerProps={readerProp}
                                //toggleDieReaderSelection={toggleReaderSelection}
                            />
                        </motion.div>
                    ))}
                </AnimatePresence>
            </motion.div>
            <div className='tray-tools'>
                <button className='button-prime'>Click</button>
                <button className='button-destructive'>Click</button>
            </div>
        </div>
    )
}   