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
    id: number,
    reader_id: number,
    total_faces: number,
    die_label: string,
    current_face: number
}

///Represents a roll request to pass to a dice-tray which will call the appHandle to create the die readers in WASM.
export interface DiceRequest{
    dieId: number,
    dieCount: number
}


///New die data. Used to prompt the application to create a new die.
export interface NewDieRequest{
    label: string,
    sides: number,
    variance: number
} 