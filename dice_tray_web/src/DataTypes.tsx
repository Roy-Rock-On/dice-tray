///Represents a die along with data about how/what is selected.
export interface DieProps {
    id: number;
    isSelected: boolean;
    dieCount: number;
    dieDetails: DieDetails;
}

///Represents a die. Holds dice information from WASM.
export interface DieDetails {
    id: number;  
    label: string;
    faces: number;
    current_face: number;
}

///Represents a die reader along with the selected status of the die reader.
export interface DieReaderProps{
    id: number,
    isSelected: boolean,
    readerDetails: DieReaderDetails
}

///Represents a die reader. Holds dice information from WASM.
export interface DieReaderDetails {
    reader_id: number,
    total_faces: number,
    die_label: string,
    current_face: number
}

///Represents a roll request to pass to a dice-tray which will call the appHandle to create the die readers in WASM.
export interface NewDieRequest{
    label: string,
    sides: number,
    variance: number 
}

export interface TrayProps{
    trayId: string;
    isSelected: boolean;
    readerProps: DieReaderProps[];
    rollTray: () => void; 
}

export function spreadTrayDetails(trayProps: TrayProps, readerDetails: DieReaderDetails[]): TrayProps{
    const readerLookup = new Map<number, DieReaderDetails>();
    readerDetails.forEach((detail) => {
        readerLookup.set(detail.reader_id, detail);
    })

    const filteredReaderProps = trayProps.readerProps.flatMap(prev => {
        const newDetails = readerLookup.get(prev.id);
        if(newDetails){
            return {
                ...prev,
                readerDetails: newDetails
            }
        }
        else{
            return [];
        }
    });

    const newReaderProps: DieReaderProps[] = Array.from(readerLookup.values()).map(newDetails => ({
        id: newDetails.reader_id,
        isSelected: false,
        readerDetails: newDetails
    }));

    const dieReaderProps = [...filteredReaderProps, ...newReaderProps];

    return {
        ...trayProps,
        readerProps: dieReaderProps 
    }
}
