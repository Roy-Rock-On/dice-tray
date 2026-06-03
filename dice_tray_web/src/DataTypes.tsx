
///Represents a die or a die reader. Holds dice information from WASM.
export interface DieProps {
    id: number;  
    label: string;
    faces: number;
    current_face: number;
    result: string;
}