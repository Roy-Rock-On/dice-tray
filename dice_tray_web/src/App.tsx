import { useState, useEffect, useRef} from 'react'
import init, { DiceAllocatorHandle } from '../pkg/dice_wasm';
import { DiceTrayAllocator } from './DiceTrayAllocator';
import './App.css'

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

  ///Return component.
  if (!wasmReady || !globalAppHandle) {
    return (
      <div>
        <h1>Dice Tray!</h1>
        <p>Loading...</p>
      </div>
    )
  }
  else{
    return (
      <div id='root'>
        <h1>Dice Tray!</h1>
        <DiceTrayAllocator appHandle={globalAppHandle} />
      </div>
    )
  }
}
 
export default App
