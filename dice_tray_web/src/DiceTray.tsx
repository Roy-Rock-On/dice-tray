import { DieReaderProps, DiceRequest } from "./DataTypes";
import { DieReader } from "./DieReader";
import { AnimatePresence, motion } from "motion/react";

interface trayProps{
    trayId: string;
    dieReaders: DieReaderProps[];
    rollRequest: DiceRequest[];
}

export function DiceTray(props: trayProps){
    
}   