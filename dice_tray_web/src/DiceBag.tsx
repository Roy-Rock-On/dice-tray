import { useState, useEffect, useRef } from 'react';
import { useDiceTray } from './DiceTrayContext'
import { DieProps, Die } from './Die'
import { genSeed } from './Utility';

interface DiceList{
    dice : DieProps[];
}

function DiceBag() {

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
                    <div key={die_summary.id}>
                        <Die {...die_summary} />
                    </div>
                ))}
            </div> 
        )

    }
}

export default DiceBag;