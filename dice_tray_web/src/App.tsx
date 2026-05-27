import { useState, useEffect } from 'react'
import init, { DiceAllocatorHandle, init_panic_hook } from '../pkg/dice_wasm';
import DiceBag from './DiceBag'
import { motion } from "motion/react"

import './App.css';

function App() {
  const [wasmReady, setWasmReady] = useState<boolean>(false);
  const [appHandle, setAppHandle] = useState<DiceAllocatorHandle | null>(null);

  useEffect(() => {
    const initWasm = async () => {
      try {
        await init(); // Initialize WASM module first
        init_panic_hook();
        setWasmReady(true);
        const handle = new DiceAllocatorHandle();
        setAppHandle(handle);
      } catch (error) {
        console.error("Failed to initialize WASM:", error);
      }
    };
    
    initWasm();
  }, []);

  if (!wasmReady || !appHandle) {
    return (
      <div className="board">
        <h1>Dice Tray!</h1>
        <p>Loading WASM...</p>
      </div>
    )
  }
  else{
    return (
      <div className="board">
        <h1>Dice Tray!</h1>
        <DiceBag appHandle={appHandle}/>
      </div>
    )
  }
}
 
export default App
