import { DieView } from './DieView'
import { DieData } from './DieDataTypes';
import { motion, AnimatePresence, MotionConfig, Reorder} from 'motion/react';

interface DiceBagProps{
    diceData: DieData[];
    isLoaded: boolean;
    toggleDieSelection: (id: number) => void;
    triggerBagRoll: () => void;
    setDieCount: (id: number, newCount: number) => void;
    openNewDieModal: () => void;
    destroyDice: () => void;
}

export function DiceBag(props: DiceBagProps) {
    if(!props.isLoaded){
        return (
            <h1>Loading...</h1>
        )
    }

    return (
        <div className='dice-bag'>
            <AnimatePresence mode="popLayout">
                {props.diceData.map((dieData)=>(
                    <motion.div
                        key={dieData.id}
                        layout
                        exit={{opacity: 0, scale: 0.9}}
                        transition={{ type: "spring", stiffness: 500, damping: 30 }}
                    >
                        <DieView 
                            dieData={dieData} 
                            toggleDieSelection={props.toggleDieSelection} 
                            setDieCount={props.setDieCount}
                        />    
                    </motion.div>
                ))}
            </AnimatePresence>
            <button 
                className='button-prime'
                onClick={props.triggerBagRoll}
            >
                ROLL
            </button>
            <button
                className='button-prime'
                onClick={props.openNewDieModal}
            >
                NEW DIE
            </button>
            <button
                className='button-destructive'
                onClick={props.destroyDice}
            >
                DESTROY
            </button>
        </div> 
    )
}
