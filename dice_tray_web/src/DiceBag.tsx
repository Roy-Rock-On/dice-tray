import { DieView } from './DieView'
import { DieData } from './DieDataTypes';
import { AnimatePresence, Reorder} from 'motion/react';

interface DiceBagProps{
    diceData: DieData[];
    isLoaded: boolean;
    toggleDieSelection: (id: number) => void;
    triggerBagRoll: () => void;
    setDieCount: (id: number, newCount: number) => void;
    openNewDieModal: () => void;
    destroyDice: () => void;
    setDiceBag: React.Dispatch<React.SetStateAction<DieData[]>>;
    onRollComplete: (dieId: number) => void;
}

export function DiceBag(props: DiceBagProps) {
    if(!props.isLoaded){
        return (
            <h1>Loading...</h1>
        )
    }

    return (
        <div className='dice-bag'>
            <Reorder.Group 
                as="div"
                axis="y" // Use "y" if your dice stack vertically, or leave it if it's a grid (see note below)
                values={props.diceData} 
                onReorder={props.setDiceBag} // 2. This callback must update your state array
                className="dice-container" // You can style this like your old container if needed
            >
                <AnimatePresence mode="sync">
                    {props.diceData.map((dieData)=>(
                        <Reorder.Item
                            as="div"
                            key={dieData.id}
                            value={dieData}
                            layout
                            exit={{opacity: 0, scale: 0.9}}
                            transition={{ type: "spring", stiffness: 500, damping: 30 }}
                        >
                            <div style={{ width: 60, height: 60, overflow: 'visible', gap: 12 }}>
                                <DieView 
                                    dieData={dieData} 
                                    toggleDieSelection={props.toggleDieSelection} 
                                    setDieCount={props.setDieCount}
                                    onRollComplete={props.onRollComplete}
                                /> 
                            </div>   
                        </Reorder.Item>
                    ))}
                </AnimatePresence>
            </Reorder.Group>
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
