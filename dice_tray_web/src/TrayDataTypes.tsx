import { DiceAction } from "./DieDataTypes";

///Represents a die reader along with the selected status of the die reader.
export interface DieReaderData{
    isSelected: boolean,
    action: DiceAction,
    readerDetails: DieReaderDetails
}

///Represents a die reader. Holds dice information from WASM.
export interface DieReaderDetails {
    reader_id: number,
    total_faces: number,
    die_label: string,
    current_face: number
}

///Represents a request to make a new tray. Very simple for now.
export interface NewTrayRequest{
    label: string
}

export interface TrayData{
    trayId: string;
    isSelected: boolean;
    readerData: DieReaderData[];
}

///Takes DieReaderDetails and spreads them out into trayData. Returning new tray data to trigger updates. 
export function spreadTrayDetails(trayData: TrayData, readerDetails: DieReaderDetails[], rolledDice: number[]): TrayData{
    const readerLookup = new Map<number, DieReaderDetails>();
    readerDetails.forEach((detail) => {
        readerLookup.set(detail.reader_id, detail);
    })

    const filteredReaderProps = trayData.readerData.flatMap(prev => {
        const newDetails = readerLookup.get(prev.readerDetails.reader_id);
        if(newDetails){
            readerLookup.delete(prev.readerDetails.reader_id);
            return {
                ...prev,
                action: rolledDice.includes(newDetails.reader_id) ? DiceAction.Roll : DiceAction.None,
                readerDetails: newDetails
            }
        }
        else{
            return [];
        }
    });

    const newReaderProps: DieReaderData[] = Array.from(readerLookup.values()).map(newDetails => ({
        id: newDetails.reader_id,
        action: rolledDice.includes(newDetails.reader_id) ? DiceAction.Roll : DiceAction.None,
        isSelected: false,
        readerDetails: newDetails
    }));

    const dieReaderData = [...filteredReaderProps, ...newReaderProps];

    return {
        ...trayData,
        readerData: dieReaderData 
    }
}
