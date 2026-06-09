import { useState, useEffect } from "react";
import { DieProps } from "./DataTypes";

interface trayProps{
    trayId: number;
    diceList: DieProps[];
}

export function DiceTray(props: trayProps){
    return (
        <div className="tray" />
    )
}