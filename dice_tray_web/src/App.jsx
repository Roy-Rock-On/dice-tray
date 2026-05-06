import { useState, useEffect } from 'react'
import init, { DiceAllocatorHandle } from '../pkg/dice_wasm';
import DiceBag from './Components';

import './App.css'

function App() {
  const [wasmReady, setWasmReady] = useState(false);
  const [appHandle, setAppHandle] = useState(null);

  useEffect(() => {
    const initWasm = async () => {
      try {
        await init(); // Initialize WASM module first
        setWasmReady(true);
        const handle = new DiceAllocatorHandle();
        setAppHandle(handle);
      } catch (error) {
        console.error("Failed to initialize WASM:", error);
        setGreeting("Failed to load WASM");
      }
    };
    
    initWasm();
  }, []);

  if (!wasmReady) {
    return (
      <div className="board">
        <h1>Dice Tray!</h1>
        <p>Loading WASM...</p>
      </div>
    )
  }

  return (
   <div className="board">
    <h1>Dice Tray!</h1>
    <DiceBag appHandle={appHandle}></DiceBag>
   </div>
  )
}

export default App
