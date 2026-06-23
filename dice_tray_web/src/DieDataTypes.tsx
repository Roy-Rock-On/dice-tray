///Used to animate the dice based on what they are doing- can be extended for more animations.
export enum DiceAction{
    None,
    Roll, 
}

///Represents a die along with data about how/what is selected.
export interface DieData {
    id: number;
    isSelected: boolean;
    action: DiceAction;
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

///Represents a request to a tray to roll new die readers. 
export interface RollRequest{
    lastRequestId: number,
    request: ReaderRequest[]
}

///An array of reader requests to add dice readers to a tray. 
export interface ReaderRequest{
    dieId: number,
    dieCount: number
}

export function spreadDieDetails(dieData: DieData[], dieDetails: DieDetails[], rolledDice: number[]): DieData[]{
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
                    action: rolledDice.includes(prev.id) ? DiceAction.Roll : DiceAction.None,
                    dieDetails: newDetails
                }
            }
            else {
                return [];
            }
        })

        const newDiceData: DieData[] = Array.from(dieLookup.values()).map(newDetails => ({
            id: newDetails.id,
            action: rolledDice.includes(newDetails.id) ? DiceAction.Roll : DiceAction.None,
            isSelected: false,
            dieCount: 0,
            dieDetails: newDetails
        }));

        return [...filteredData, ...newDiceData]
}

export function getRollRequest(dieData: DieData[], requestId: number): RollRequest {
    const readerRequests: ReaderRequest[]  = dieData.flatMap((die) => {
        if (die.isSelected){
            return {
                dieId: die.id,
                dieCount: die.dieCount
            };
        }
        return [];
    });
    return  {
        lastRequestId: requestId,
        request: readerRequests
    }
}