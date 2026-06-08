import { useState, useEffect, useRef } from 'react';
import { useDiceTray } from './DiceTrayContext'
import { DieView } from './DieView'
import { genSeed } from './Utility';
import { DieProps } from './DataTypes';

interface DiceList{
    dice : DieProps[];
}

function DiceBag() {
    let selectedDieIds : Number[] = []; 

    const selectDie = (dieID: Number, isSelected: boolean) => {
        console.log("Select die is triggering!");
        if (isSelected){
            selectedDieIds = [...selectedDieIds, dieID]
        }
        else{
            selectedDieIds = selectedDieIds.filter(id => dieID !== id);
        }
        console.log("Currently selected = " + selectedDieIds);
    }

    const appHandle = useDiceTray();

    const hasInit = useRef(false);
    const [diceList, setDiceList] = useState<DiceList>({ dice: [] });
    const [isLoading, setIsLoading] = useState(true);

    useEffect (() => {
        //clause to prevent double fire.
        if (hasInit.current) return;
        hasInit.current = true;

        const addDice = async () => {
            try{
                //Generate a set of dice to play with.
                appHandle.create_die(4, genSeed());
                appHandle.create_die(6, genSeed());
                appHandle.create_die(8, genSeed());
                appHandle.create_die(10, genSeed());
                appHandle.create_die(12, genSeed());
                appHandle.create_die(20, genSeed());
                appHandle.create_die(100, genSeed());

                let diceList = appHandle.get_dice_data() as DiceList;
                console.log("dice data = " + diceList);
                
                setDiceList(diceList);

            }catch(error){
                console.error("Caught error while creating dice: ", error);
            }finally{
                setIsLoading(false);
            }
        };

        addDice();
    });
    
    if (isLoading){
        return (
            <div className="dice-bag">
                <h1>Loading Dice...</h1>
            </div>
        )  
    }
    else{
        return (
            <div className="dice-bag">
                {diceList.dice.map((die_summary)=>(
                    <div
                        className='dice-space'
                        key={die_summary.id}>
                        <DieView dieProps={{...die_summary}} selectDie={selectDie} />
                    </div>
                ))}
            </div> 
        )

    }
}

export default DiceBag;