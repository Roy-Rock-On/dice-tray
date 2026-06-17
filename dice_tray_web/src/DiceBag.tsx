import { DieView } from './DieView'
import { DieProps } from './DataTypes';
import { motion, AnimatePresence, MotionConfig, Reorder} from 'motion/react';

interface DiceBagProps{
    diceProps: DieProps[];
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
                {props.diceProps.map((dieProp)=>(
                    <motion.div
                        key={dieProp.id}
                        layout
                        exit={{opacity: 0, scale: 0.9}}
                        transition={{ type: "spring", stiffness: 500, damping: 30 }}
                    >
                        <DieView 
                            dieProps={dieProp} 
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
