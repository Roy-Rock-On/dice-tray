mod wasm_tray;

use rust_dice::dice::Die32;
use wasm_tray::{WasmTray, TrayData};
use wasm_bindgen::prelude::*;
use serde_json;
use web_sys;

#[wasm_bindgen]
pub fn greet(msg: &str) -> String{
    format!("{}", msg)
}

#[wasm_bindgen]
pub struct DiceTrayHandle {
    tray: WasmTray,
}

#[wasm_bindgen]
impl DiceTrayHandle {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            tray: WasmTray::new(0)
        }
    }

    /// Add a die to the tray
    #[wasm_bindgen]
    pub fn add_die(&mut self, sides: u32) -> Result<(), JsValue> {
        // Validate input
        if sides == 0 {
            return Err(JsValue::from_str("Die must have at least 1 side"));
        }
        if sides > 10000 {
            return Err(JsValue::from_str("Die cannot have more than 10,000 sides"));
        }
        
        // Log for debugging
        web_sys::console::log_1(&format!("Creating die with {} sides", sides).into());
        
        let new_id = self.tray.get_next_die_id();
        web_sys::console::log_1(&format!("Using die ID: {}", new_id).into());
        
        let new_die = Die32::new(new_id, None, sides, None);
        web_sys::console::log_1(&"Die created successfully".into());
        
        self.tray.add_die(new_die);
        web_sys::console::log_1(&"Die added to tray successfully".into());
        
        Ok(())
    }

    /// Roll all dice in the tray
    #[wasm_bindgen]
    pub fn roll_all(&mut self) -> Result<(), JsValue> {
        self.tray.roll_all();
        Ok(())
    }

    /// Clear all dice from the tray
    #[wasm_bindgen]
    pub fn clear(&mut self) -> Result<(), JsValue> {
        self.tray.clear();
        Ok(())
    }

    ///Gets the tray data used to update the tray.
    #[wasm_bindgen]
    pub fn get_tray_data(&self) -> Result<String, JsValue> {
        let tray_data = TrayData::from_tray(&self.tray);
        serde_json::to_string(&tray_data).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
