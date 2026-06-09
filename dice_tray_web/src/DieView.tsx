import {memo, useState, useEffect, useRef, useCallback} from "react";
import { useDiceTray } from "./DiceTrayContext";
import { DieShape } from "./DieShape";
import { motion } from "framer-motion";
import { DieProps } from "./DataTypes";
import { useAnimation } from "framer-motion";
import { text } from "motion/react-client";

interface DieViewProps {
    dieProps: DieProps,
    rollCount: number,
    selectDie: (id: number, isSelected: boolean) => void;
}

const dieVariants = {
    selected: {
        scale: 1.20,
        stroke: '#ffffff'
    },
    unselected: {
        scale: 1,
        stroke: '#000000'
    }
}

const dieTextVariants ={
    rolling: {
        opacity: 0,
        transition: {duration: 0.1}
    },
    static: {
        opacity : 1,
        transition: {duration: 0.1}
    }
}


function DieViewComponent(props: DieViewProps) {
    const appHandle = useDiceTray();
    const [isSelected, setIsSelected] = useState<boolean>(false);
    const [dieState, setDieState] = useState<DieProps>(props.dieProps);
    
    /*
    useEffect(() => {
        setDieState(props.dieProps);
    }, [props.dieProps]);
    */

    const anim = useAnimation();
    useEffect(() => {
        anim.start(isSelected ? "selected" : "unselected");
    }, [isSelected, anim])

    const [isRolling, setIsRolling] = useState<boolean>(false);
    
    useEffect(() => {
        const handleRoll = async () => {
            setIsRolling(true);
            try{
                let nextState = appHandle.roll_die(dieState.id) as DieProps;
                await anim.start({
                    rotate: [0, 180, 360],
                    x: [0, -5, 5, 0],
                    transition: {duration: 0.4, ease: "easeIn"}
                });

                setDieState(nextState);

                await anim.start({
                    scale: [1, 1.2, 1],
                    transition: { duration: 0.2 }
                });
            }
            catch(error){
                console.log("Error rolling die:", error);
            }finally{
                setIsRolling(false);
            }
        }

        if(!isSelected || isRolling) return;
        handleRoll();
    }, [props.rollCount]);

    const toggleSelect = (() => {
        console.log("Toggling select for die = " + dieState.id);
        const nextSelect = !isSelected;
        setIsSelected(nextSelect);
        props.selectDie(dieState.id, nextSelect);
    });

    return (
        <motion.svg 
            animate={isSelected ? "selected" : "unselected"}
            variants={dieVariants}
            whileHover={{ scale: 1.05}}
            stroke-width = {2}
            width={60}
            height={60}
            viewBox="0 0 110 110"
            style={{
                background: 'transparent',
                overflow: 'visible'
            }}

            onClick={toggleSelect}
            whileTap={{scale:1.15}}
            transition={{
                type:"spring",
                stiffness: 400,
                damping: 15
            }}
        >
            <motion.g animate={anim}>
                <DieShape dieFaces={dieState.faces} dieColor="#1885cf"/>
                <motion.text
                    animate={isRolling ? "rolling" : "static"}
                    variants={dieTextVariants}
                    x="50" 
                    y="50"
                    stroke-width={0} 
                    fill= {'#000305'}
                    fontSize="24" 

                    fontWeight="bold"
                    textAnchor="middle" 
                    dominantBaseline="central"
                    style={{ userSelect: 'none' }}  
                >
                    {dieState.current_face}
                </motion.text>
            </motion.g>
        </motion.svg>
    );
}

export const DieView = memo(DieViewComponent);