import { useState, useEffect, useRef } from 'react'
import init, { DiceAllocatorHandle } from '../pkg/dice_wasm';
import { DiceBag } from './DiceBag'
import { DiceTrayProvider } from './DiceTrayContext'

import './App.css';

let globalAppHandle: DiceAllocatorHandle | null = null;

function App() {
  const isInitializing = useRef(false);
  const [wasmReady, setWasmReady] = useState<boolean>(false);

  useEffect(() => {
    if(globalAppHandle){
      setWasmReady(true);
      return;
    }

    if (isInitializing.current) return;
    isInitializing.current = true;


    const initWasm = async () => {
      try {
        await init();
        globalAppHandle = new DiceAllocatorHandle;
        setWasmReady(true);
      } catch (error) {
        console.error("Failed to initialize WASM:", error);
      } finally {
        isInitializing.current = false;
      }
    };
    
    initWasm();
  }, []);

  if (!wasmReady || !globalAppHandle) {
    return (
      <div className="board">
        <h1>Dice Tray!</h1>
        <p>Loading WASM...</p>
      </div>
    )
  }
  else{
    return (
      <DiceTrayProvider appHandle={globalAppHandle}>
        <h1>Dice Tray!</h1>    
        <div className="board">
          <DiceBag />
          <div className="tray-board">

          </div>
        </div>
      </DiceTrayProvider>
    )
  }
}
 
export default App
