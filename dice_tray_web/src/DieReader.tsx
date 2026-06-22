import { memo, useState, useEffect } from "react";
import { DieShape } from "./DieShape";
import { motion } from "framer-motion";
import { DieReaderData } from "./TrayDataTypes";
import { useAnimation } from "framer-motion";
import { DiceAction } from "./DieDataTypes";

interface DieReaderProps {
   readerData: DieReaderData,
   toggleSelection: (id: number) => void; 
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

const dieTextVariants = {
    rolling: {
        opacity: 0,
        transition: { duration: 0.1 }
    },
    static: {
        opacity: 1,
        transition: { duration: 0.1 }
    }
}

function DieReaderComponent(props: DieReaderProps) {
    const [dieResult, setDieResult] = useState<number>(props.readerData.readerDetails.current_face)
    const [isRolling, setIsRolling] = useState<boolean>(false);

    useEffect(() => {
        const handleRoll = async () => {
            setIsRolling(true);
            try {
                let nextValue = props.readerData.readerDetails.current_face;
                await anim.start({
                    rotate: [0, 180, 360],
                    x: [0, -5, 5, 0],
                    transition: { duration: 0.4, ease: "easeIn" }
                });

                setDieResult(nextValue);

                await anim.start({
                    scale: [1, 1.2, 1],
                    transition: { duration: 0.2 }
                });
            }
            catch (error) {
                console.log("Error rolling die:", error);
            } finally {
                setIsRolling(false);
            }
        }

        if (props.readerData.action !== DiceAction.Roll || isRolling) return;
        handleRoll();
    }, [props.readerData.readerDetails]);

    const anim = useAnimation();
    useEffect(() => {
        anim.start(props.readerData.isSelected ? "selected" : "unselected");
    }, [props.readerData.isSelected, anim])

    const toggleSelect = (() => {
        props.toggleSelection(props.readerData.readerDetails.reader_id);
    });

    return (
        <motion.svg
            animate={props.readerData.isSelected ? "selected" : "unselected"}
            variants={dieVariants}
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
            <motion.g animate={anim}>
                <DieShape dieFaces={props.readerData.readerDetails.total_faces} dieColor="#1885cf" />
                <motion.text
                    animate={isRolling ? "rolling" : "static"}
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
                    {dieResult}
                </motion.text>
            </motion.g>
        </motion.svg>
    );
}

export const DieReader = memo(DieReaderComponent);