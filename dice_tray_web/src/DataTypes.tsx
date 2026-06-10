
///Represents a die or a die reader. Holds dice information from WASM.
export interface DieProps {
    id: number;  
    label: string;
    faces: number;
    current_face: number;
}

export interface DieReaderState {
    die_id: number,
    reader_id: number,
    total_faces: number,
    die_label: string,
    current_face: number
}

///Represents a roll request to pass to a dice-tray which will call the appHandle to create the die readers in WASM.
export interface RollRequest{
    dieId: number;
    count: number;
}