import { useEffect, useState} from "react";

export interface DieProps {
    die_id: number;
    faceCount: number;
    currentFace: number;
    result: number;
    label: string;
}

export function Die(props: DieProps) {
    const [dieProps, setDieProps] = useState<DieProps>(props);

    return (
        <div className="die">
            <p>{dieProps.currentFace}</p>
        </div>
    );
}