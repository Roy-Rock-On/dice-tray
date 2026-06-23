import { DiceAction, ReaderRequest } from "./DieDataTypes";

///Represents a die reader along with the selected status of the die reader.
export interface DieReaderData{
    isSelected: boolean,
    action: DiceAction,
    readerDetails: DieReaderDetails
}

///Represents a die reader. Holds dice information from WASM.
export interface DieReaderDetails {
    die_id: number,
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
    readerRequest: ReaderRequest[];
    //readerData: DieReaderData[];
}

///Takes DieReaderDetails and spreads them out into trayData. Returning new tray data to trigger updates. 


export function spreadReaderDetails(oldReaderData: DieReaderData[], newReaderDetails: DieReaderDetails[], rolledDice: number[]): DieReaderData[]{
    const detailLookup = new Map<number, DieReaderDetails>();
    newReaderDetails.forEach((detail) => {
        detailLookup.set(detail.reader_id, detail);
    })


    const filteredReaderData = oldReaderData.flatMap(prev => {
        const newDetails = detailLookup.get(prev.readerDetails.reader_id);
        if(newDetails){
            detailLookup.delete(prev.readerDetails.reader_id);
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

    const newReaderData: DieReaderData[] = Array.from(detailLookup.values()).map(newDetails => ({
        id: newDetails.reader_id,
        action: rolledDice.includes(newDetails.reader_id) ? DiceAction.Roll : DiceAction.None,
        isSelected: false,
        readerDetails: newDetails
    }));

    return [...filteredReaderData, ...newReaderData];
}
