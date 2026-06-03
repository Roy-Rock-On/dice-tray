import { useState} from "react";
import { useDiceTray } from "./DiceTrayContext";
import { DieShape } from "./DieShape";
import { motion } from "framer-motion";
import { DieProps } from "./DataTypes";

interface DieViewProps {
    dieProps: DieProps,
    selectDie: (id: Number, isSelected: boolean) => void;
}

export function DieView(props: DieViewProps) {
    const appHandle = useDiceTray();
    const [isSelected, setIsSelected] = useState<Boolean>(false);
    const [dieProps, setDieProps] = useState<DieProps>(props.dieProps);

    let isRolling = false;
    
    const rollDie = (() => {
        if(isRolling) return;
        isRolling = true;
        try{
            let summary = appHandle.roll_die(dieProps.id) as DieProps;
            setDieProps(summary);
        }
        catch(error){
            console.log("Error rolling die:", error);
        }finally{
            isRolling = false;
        }
    });

    const toggleSelect = (() => {
        console.log("Toggling select for die = " + dieProps.id);
        if (isSelected) {
            setIsSelected(false);
            props.selectDie(dieProps.id, false);
        }
        else{
            setIsSelected(true);
            props.selectDie(dieProps.id, true);
        }
    });

    return (
        <motion.svg 
            animate={{
                scale: isSelected ? 1.25 : 1,
                stroke: isSelected ? '#ffffff' : "#000000"
            }}

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
            <DieShape dieFaces={dieProps.faces} dieColor="#1885cf"/>
            <text
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
                {dieProps.current_face}
            </text>
        </motion.svg>
    );
}