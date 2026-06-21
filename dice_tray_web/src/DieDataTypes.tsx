///Represents a die along with data about how/what is selected.
export interface DieData {
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

///Represents a roll request to pass to a dice-tray which will call the appHandle to create the die readers in WASM.
export interface NewDieRequest{
    label: string,
    sides: number,
    variance: number 
}

export function spreadDieDetails(dieData: DieData[], dieDetails: DieDetails[]): DieData[]{
    const dieLookup = new Map<number, DieDetails>();
        dieDetails.forEach((detail) => {
            dieLookup.set(detail.id, detail);
        });

        const filteredData =  dieData.flatMap(prev => {
            const newDetails = dieLookup.get(prev.id);
            if (newDetails){
                dieLookup.delete(prev.id);
                return {
                    ...prev,
                    dieDetails: newDetails
                }
            }
            else {
                return [];
            }
        })

        const newDiceData: DieData[] = Array.from(dieLookup.values()).map(newDetails => ({
            id: newDetails.id,
            isSelected: false,
            dieCount: 0,
            dieDetails: newDetails
        }));

        return [...filteredData, ...newDiceData]
}