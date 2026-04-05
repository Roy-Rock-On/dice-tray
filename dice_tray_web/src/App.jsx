import { useState, useEffect } from 'react'
import init, { greet, DiceTrayHandle } from '../pkg/dice_wasm';

import './App.css'

function App() {
  const [wasmReady, setWasmReady] = useState(false);
  const [greeting, setGreeting] = useState("Loading...");

  useEffect(() => {
    const initWasm = async () => {
      try {
        await init(); // Initialize WASM module first
        setGreeting(greet("React App"));
        setWasmReady(true);
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
        <h2>{greeting}</h2>
        <p>Loading WASM...</p>
      </div>
    )
  }

  return (
   <div className="board">
    <h1>Dice Tray!</h1>
    <h2>{greeting}</h2>
    <Tray />
   </div>
  )
}

function Tray(){
  const [trayHandle, setTrayHandle] = useState(null);
  const [newFaces, setNewFaces] = useState(6);
  const [trayData, setTrayData] = useState(null);
  const [diceArray, setDiceArray] = useState([]);

  useEffect(() => {
    const initTray = () => {
      try {
        console.log("Initializing DiceTrayHandle.");
        const handle = new DiceTrayHandle();
        setTrayHandle(handle);
        // get_tray_data() now returns a JSON string
        const jsonString = handle.get_tray_data();
        setTrayData(jsonString);
      } catch (error) {
        console.error("Failed to initialize tray handle:", error);
      }
    };
    initTray();
  }, []);

  useEffect(() => {
    const get_die_array = () => {
      try {
        if (trayData) {
          // trayData is now a JSON string from Rust
          const parsedData = JSON.parse(trayData);
          
          // Extract dice array from the parsed data
          if (parsedData && Array.isArray(parsedData.dice)) {
            setDiceArray(parsedData.dice);
          } else {
            console.warn('Unexpected tray data format:', parsedData);
            setDiceArray([]);
          }
        } else {
          setDiceArray([]);
        }
      } catch (error) {
        console.error('Failed to parse tray data:', error);
        setDiceArray([]);
      }
    };
    
    get_die_array();
  }, [trayData])

  const AddDie = () => {
    if (!trayHandle) return;
    
    try {
      console.log("Adding a die to the tray.");
      trayHandle.add_die(newFaces);
      // get_tray_data() now returns a JSON string
      const jsonString = trayHandle.get_tray_data();
      setTrayData(jsonString);
    } catch (error) {
      console.error("Failed to add die:", error);
    }
  };

  if (!trayHandle){
    return(
      <p>Tray is loading...</p>
    )
  }
  
  return(
    <div>
      <div className="tray">
        {diceArray.map((die, index) => (
          <Die key={index} dieData={die} />
        ))}
      </div>
      <div className="toolbar">
          <input 
            type="number"
            value={newFaces}
            onChange={(e) => {
              const value = Number(e.target.value);
              if (value > 0 && value < 10000) {
                setNewFaces(value);
              } else {
                console.error("Value must be a positive number less than 10,000");
              }
            }} 
            placeholder="faces" 
          />
          <button onClick={AddDie}>Add Die</button>
      </div>
    </div>
  )
}

function Die({ dieData }){
  return(
    <div>
      <div className ="dice">
        <p className="dice-text">{dieData.current_result}</p>
      </div>
    </div>
  )
}

export default App
