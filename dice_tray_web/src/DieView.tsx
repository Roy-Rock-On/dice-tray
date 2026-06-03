import { useState} from "react";
import { useDiceTray } from "./DiceTrayContext";
import { DieShape } from "./DieShape";
import { motion } from "framer-motion";
import { DieProps } from "./DataTypes";

interface DieViewProps {
    dieProps: DieProps,
}


export function DieView(props: DieViewProps) {
    const appHandle = useDiceTray();
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

    return (
        <motion.svg 
            whileHover={{ scale: 1.05}}
            whileDrag={{
                scale: 1.1, 
                cursor: "grabbing"
            }}

            width={60}
            height={60}
            viewBox="0 0 100 100"
            style={{background: 'transparent'}}
            onClick={rollDie}
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
                fill= {'#000305'}
                fontSize="18" 
                fontWeight="bold"
                textAnchor="middle" 
                dominantBaseline="central"
                style={{ userSelect: 'none' }} // Prevents accidental text highlighting on click   
            >
                {dieProps.current_face}
            </text>
        </motion.svg>
    );
}