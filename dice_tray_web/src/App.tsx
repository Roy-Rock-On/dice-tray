import { useState, useEffect } from 'react'
import init, { DiceAllocatorHandle } from '../pkg/dice_wasm';
import { createBox, motion } from "motion/react"

import './App.css';

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
  else{
    return (
      <div className="board">
        <h1>Dice Tray!</h1>
        <div className="tray">
          <motion.div
            style={box}
            whileHover={{scale: 1.2}}
            whileTap={{scale:0.8}}
            initial={{opacity: 0, scale: 0}}
            animate={{rotate: 360, scale: 1, opacity: 1}}
            transition={{duration: 1, repeat: 0}}
          />
        </div>
      </div>
    )
  }
}

const box = {
    align: "center",
    width: 100,
    height: 100,
    backgroundColor: "red",
    borderRadius: 5,
}
 

export default App
