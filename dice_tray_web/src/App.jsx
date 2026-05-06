import { useState, useEffect } from 'react'
import init, { greet, DiceAllocatorHandle } from '../pkg/dice_wasm';

import './App.css'

function App() {
  const [wasmReady, setWasmReady] = useState(false);
  const [greeting, setGreeting] = useState("Loading...");

  useEffect(() => {
    const initWasm = async () => {
      try {
        await init(); // Initialize WASM module first
        setGreeting(greet("New Tray"));
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
    const initTray = async () => {
      try {
        console.log("Initializing DiceTrayHandle.");
        const handle = new DiceTrayHandle();
        setTrayHandle(handle);
        const jsonString = await handle.get_tray_data();
        setTrayData(jsonString);
      } catch (error) {
        console.error("Failed to initialize tray handle:", error);
      }
    };
    initTray();
  }, []);

  useEffect(() => {
    const get_die_array = async () => {
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

  const AddDie = async () => {
    if (!trayHandle) return;
    
    try {
      console.log("Adding die with", newFaces, "faces");
      
      // Add the die
      await trayHandle.add_die(newFaces);
      console.log("Die added successfully");
      // Get updated tray data
      const jsonString = await trayHandle.get_tray_data();
      console.log("Tray data retrieved successfully");
      
      setTrayData(jsonString);
    } catch (error) {
      console.error("Failed to add die:", error);
      console.error("Error details:", {
        name: error.name,
        message: error.message,
        stack: error.stack
      });
    }
  };

  const ClearTray = async () => {
    if (!trayHandle) return;
    try {
      console.log("Clearing tray");
      
      await trayHandle.clear();
      console.log("Tray cleared successfully");     
      const jsonString = await trayHandle.get_tray_data();
      console.log("Tray data retrieved after clear");
      
      setTrayData(jsonString);
    } catch (error) {
      console.error("Failed to clear tray:", error);
      console.error("Error details:", {
        name: error.name,
        message: error.message,
        stack: error.stack
      });
    }
  };

  const RollAll = async () => {
    if (!trayHandle) return;
    try {
      console.log("Rolling all dice");
      
      await trayHandle.roll_all();
      console.log("All dice rolled successfully");
      
      // Small delay to prevent race conditions
      await new Promise(resolve => setTimeout(resolve, 10));
      
      const jsonString = await trayHandle.get_tray_data();
      console.log("Tray data retrieved after roll");
      
      setTrayData(jsonString);
    } catch (error) {
      console.error("Failed to roll dice:", error);
      console.error("Error details:", {
        name: error.name,
        message: error.message,
        stack: error.stack
      });
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
          <button onClick={RollAll}>Roll All</button>
          <button onClick={ClearTray}>Clear Tray</button>
      </div>
    </div>
  )
}

function Die({ dieData }){
  console.log("Die Data is = " + JSON.stringify(dieData));
  
  // Defensive programming - handle malformed data
  if (!dieData || typeof dieData !== 'object') {
    return (
      <div>
        <div className="dice-container">
          <div className="dice">
            <p className="dice-text">?</p>
          </div>
          <p className="label-text">Error</p>
        </div>
      </div>
    );
  }
  
  const die32 = dieData.Die32;
  
  if (!die32 || typeof die32 !== 'object') {
    return (
      <div>
        <div className="dice-container">
          <div className="dice">
            <p className="dice-text">?</p>
          </div>
          <p className="label-text">Invalid</p>
        </div>
      </div>
    );
  }

  return(
    <div>
      <div className="dice-container">
        <div className ="dice">
          <p className="dice-text">{die32.current_face || '?'}</p>
        </div>
        <p className="label-text">{die32.label || 'Unknown'}</p>
      </div>
    </div>
  )
}

export default App
