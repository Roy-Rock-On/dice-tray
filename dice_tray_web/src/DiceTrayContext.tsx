import {createContext, useContext, ReactNode} from 'react';
import {DiceAllocatorHandle} from '../pkg/dice_wasm';

const DiceTrayContext = createContext<DiceAllocatorHandle | undefined>(undefined);

interface DiceTrayContextProps {
    appHandle: DiceAllocatorHandle;
    children: ReactNode;
}

export function DiceTrayProvider({appHandle, children}: DiceTrayContextProps) {
    return (
        <DiceTrayContext.Provider value={appHandle}>
            {children}
        </DiceTrayContext.Provider>
    );
}

export function useDiceTray() {
    const context = useContext(DiceTrayContext);
    if (context === undefined){
        throw new Error("Die tray context not found.");
    }
    return context;
}
