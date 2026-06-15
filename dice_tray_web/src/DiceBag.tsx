import { DieView } from './DieView'
import { DieState, DieSelection} from './DataTypes';
import { motion, AnimatePresence, MotionConfig, Reorder} from 'motion/react';

interface DiceBagProps{
    diceState: DieState[];
    diceSelection: Record<number, DieSelection>;
    isLoaded: boolean;
    toggleDieSelection: (id: number) => void;
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
                {props.diceState.map((die_state)=>(
                    <motion.div
                        key={die_state.id}
                        layout
                        exit={{opacity: 0, scale: 0.9}}
                        transition={{ type: "spring", stiffness: 500, damping: 30 }}
                    >
                        <DieView 
                            dieState={die_state} 
                            isSelected={props.diceSelection[die_state.id]?.isSelected ?? false} 
                            toggleDieSelection={props.toggleDieSelection} 
                        />    
                    </motion.div>
                ))}
            </AnimatePresence>
        </div> 
    )
}
