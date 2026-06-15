
///Represents a die or a die reader. Holds dice information from WASM.
export interface DieState {
    id: number;  
    label: string;
    faces: number;
    current_face: number;
}

///Wrapper to get the selected status of a die in the bag. 
export interface DieSelection{
    isSelected: boolean,
    dieCount: number
}

///Represents a die reader that sits in a tray.
export interface DieReaderState {
    die_id: number,
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