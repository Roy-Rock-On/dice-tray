import { useState, useEffect, useRef } from 'react';
import {DiceAllocatorHandle} from '../pkg/dice_wasm';
import { DieProps, Die } from './Die'
import { genSeed } from './Utility';

interface DiceBagProps {
    appHandle: DiceAllocatorHandle;
}

interface DiceList{
    Dice : DieProps[];
}

function DiceBag(app: DiceBagProps) {
    const hasInit = useRef(false);
    const [diceList, setDiceList] = useState<DiceList>()

    useEffect (() => {
        //clause to prevent double fire.
        if (hasInit.current) return;
        hasInit.current = true;

        const addDice = async () => {
            try{
                app.appHandle.create_die(4, genSeed());
                app.appHandle.create_die(6, genSeed());
                app.appHandle.create_die(8, genSeed());
                app.appHandle.create_die(10, genSeed());
                app.appHandle.create_die(12, genSeed());
                app.appHandle.create_die(20, genSeed());
                app.appHandle.create_die(100, genSeed());
                let diceList = app.appHandle.get_dice_data();
                console.log("dice data = " + diceList);
            }catch(error){
                console.error("Caught error while creating dice: ", error);
            }
        };

        addDice();
    }, [app]);

    return (
        <div className="dice-bag">
            <h1>Placeholder</h1>
        </div>
    )
}

export default DiceBag;