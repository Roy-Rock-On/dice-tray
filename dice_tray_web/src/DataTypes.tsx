
///Represents a die or a die reader. Holds dice information from WASM.
export interface DieProps {
    id: number;  
    label: string;
    faces: number;
    current_face: number;
    result: string;
}

///Represents a roll request to pass to a dice-tray which will call the appHandle to create the die readers in WASM.
export interface RollRequest{
    dieId: number;
    count: number;
}