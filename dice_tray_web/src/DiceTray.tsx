import { useState, useEffect, useCallback } from 'react'
import { DieReaderProps, DieReaderDetails, DiceRequest, TrayProps } from "./DataTypes";
import { DieReader } from "./DieReader";
import { AnimatePresence, motion } from "motion/react";
import { DiceAllocatorHandle } from '../pkg/dice_wasm';

export function DiceTray(props: TrayProps){
    const [readerProps, setReaderProps] = useState<DieReaderProps[]>([]);

    const updateReaderProps = (readerList: DieReaderDetails[]) => {
        setReaderProps((prevProps) => {
            const readerLookup = new Map<number, DieReaderDetails>();
            readerList.forEach((detail) => {
                readerLookup.set(detail.reader_id, detail);
            });

            const filteredProps = prevProps.flatMap(prev => {
                const newDetails = readerLookup.get(prev.id);
                if (newDetails){
                    readerLookup.delete(prev.id);
                    return {
                        ...prev,
                        readerDetails: newDetails
                    }
                }
                else {
                    return [];
                }
            })

            const newReaderProps: DieReaderProps[] = Array.from(readerLookup.values()).map(newDetails => ({
                id: newDetails.reader_id,
                isSelected: false,
                readerDetails: newDetails
            }));

            return [...filteredProps, ...newReaderProps]
        })
    }

    const toggleReaderSelection = useCallback((readerId: number) =>{
        setReaderProps((prevProps) => {
            return prevProps.map(prev => {
                if (prev.id == readerId){
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
    }, [readerProps])

    return (
        <div className='tray-group'>
            <motion.div
                className='tray'
                whileHover={{
                    scale: 1.02,
                    boxShadow: '0px 10px 30px rgba(244, 242, 247, 0.3)'
                }}
                transition={{
                    type: 'spring',
                    stiffness: 300,
                    damping: 20
                }}
            >
                <AnimatePresence mode='popLayout'>
                    {readerProps.map((readerProp) => (
                        <motion.div
                            key={readerProp.id}
                            layout
                            exit={{opacity:0, scale: 0.9}}
                            transition={{ type: "spring", stiffness: 500, damping: 30 }}
                        >
                            <DieReader
                                readerProps={readerProp}
                                toggleDieReaderSelection={toggleReaderSelection}
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