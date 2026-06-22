import {memo, useState, useEffect} from "react";
import { DieShape } from "./DieShape";
import { motion } from "framer-motion";
import { DieData, DiceAction } from "./DieDataTypes";
import { Variants } from "framer-motion";

interface DieViewProps {
    dieData: DieData,
    toggleDieSelection: (id: number) => void;
    setDieCount: (id: number, newCount: number) => void;
    onRollComplete: (dieId: number) => void;
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


function DieViewComponent(props: DieViewProps) {
    const tempBundleSelection = (() =>{
        toggleSelect();
        setDieCount();
    });

    const toggleSelect = (() => {
        console.log("Toggling select for die = " + props.dieData.dieDetails.id);
        props.toggleDieSelection(props.dieData.dieDetails.id);
    });

    const setDieCount = (() => {
        console.log("Setting die count to 1 as a test.");
        props.setDieCount(props.dieData.id, 1);
    })

    //calculate some vars to run animations
    const isRollingAction = props.dieData.action === DiceAction.Roll;
    const selectionState = props.dieData.isSelected ? "selected" : "unselected";
    const actionState = isRollingAction ? "rolling" : "static";

    return (
        <motion.svg 
            animate={[selectionState, actionState]}
            variants={dieSelectionVariants}
            whileHover={{ scale: 1.05}}
            strokeWidth = {2}
            width={60}
            height={60}
            viewBox="0 0 110 110"
            style={{
                background: 'transparent',
                overflow: 'visible'
            }}
            onClick={tempBundleSelection}
            whileTap={{scale:1.15}}
            transition={{
                type:"spring",
                stiffness: 400,
                damping: 15
            }}
        >
            <motion.g
                animate={actionState}
                variants={dieActionVariants}
                onAnimationComplete={() => {
                    if(isRollingAction){
                        props.onRollComplete(props.dieData.id);
                    }
                }}
            >
                <DieShape dieFaces={props.dieData.dieDetails.faces} dieColor="#1885cf"/>
                    <motion.text
                        animate={actionState}
                        variants={dieTextVariants}
                        x="50" 
                        y="50"
                        strokeWidth={0} 
                        fill= {'#000305'}
                        fontSize="24"
                        fontWeight="bold"
                        textAnchor="middle" 
                        dominantBaseline="central"
                        style={{ userSelect: 'none' }}  
                    >
                        {props.dieData.dieDetails.current_face}
                    </motion.text>
            </motion.g>
        </motion.svg>
    );
}

export const DieView = memo(DieViewComponent);