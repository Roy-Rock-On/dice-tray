import { memo, useState, useEffect } from "react";
import { DieShape } from "./DieShape";
import { motion } from "framer-motion";
import { DieReaderData } from "./TrayDataTypes";
import { useAnimation, Variants } from "framer-motion";
import { DiceAction } from "./DieDataTypes";

interface DieReaderProps {
   readerData: DieReaderData,
   toggleSelection: (id: number) => void; 
   onRollComplete: (readerId: number) => void;
}

const dieSelectionVariants: Variants = {
    selected: {
        scale: 1.20,
        stroke: '#ffffff'
    },
    unselected: {
        scale: 1,
        stroke: '#000000'
    }
}

const dieTextVariants: Variants ={
    rolling: {
        opacity: 0,
        transition: {duration: 0}
    },
    static: {
        opacity : 1,
        transition: {duration: 0.2}
    }
}

const dieActionVariants: Variants = {
    rolling: {
        rotate: [0, 180, 360],
        x: [0, -5, 5, 0],
        transition: {duration: 0.4, ease: "easeIn"}
    },
    static: {
        rotate: 0,
        x: 0
    }
};

function DieReaderComponent(props: DieReaderProps) {
    const toggleSelect = (() => {
        props.toggleSelection(props.readerData.readerDetails.reader_id);
    });

    const isRollingAction = props.readerData.action === DiceAction.Roll;
    const selectionState = props.readerData.isSelected ? "selected" : "unselected";
    const actionState = isRollingAction ? "rolling" : "static"

    return (
        <motion.svg
            animate={[selectionState, actionState]}
            variants={dieSelectionVariants}
            whileHover={{ scale: 1.05 }}
            strokeWidth={2}
            width={60}
            height={60}
            viewBox="0 0 110 110"
            style={{
                background: 'transparent',
                overflow: 'visible'
            }}

            onClick={toggleSelect}
            whileTap={{ scale: 1.15 }}
            transition={{
                type: "spring",
                stiffness: 400,
                damping: 15
            }}
        >
            <motion.g 
                animate={actionState}
                variants={dieActionVariants}
                onAnimationComplete={() => {
                    if(isRollingAction){
                        props.onRollComplete(props.readerData.readerDetails.reader_id);
                    }
                }}
            >
                <DieShape dieFaces={props.readerData.readerDetails.total_faces} dieColor="#1885cf" />
                <motion.text
                    animate={actionState}
                    variants={dieTextVariants}
                    x="50"
                    y="50"
                    strokeWidth={0}
                    fill={'#000305'}
                    fontSize="24"

                    fontWeight="bold"
                    textAnchor="middle"
                    dominantBaseline="central"
                    style={{ userSelect: 'none' }}
                >
                    {props.readerData.readerDetails.current_face}
                </motion.text>
            </motion.g>
        </motion.svg>
    );
}

export const DieReader = memo(DieReaderComponent);