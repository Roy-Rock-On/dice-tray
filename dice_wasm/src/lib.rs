mod wasm_tray;

use rust_dice::dice::Die32;
use wasm_tray::{WasmTray, TrayData};
use wasm_bindgen::prelude::*;
use serde_json;

#[wasm_bindgen]
pub fn greet(msg: &str) -> String{
    format!("Hello {}", msg)
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
    pub fn add_die(&mut self, sides: u32) {
        let new_id = self.tray.get_next_die_id();
        let new_die = Die32::new(new_id, None, sides, None);
        self.tray.add_die(new_die);
    }

    /// Roll all dice in the tray
    #[wasm_bindgen]
    pub fn roll_all(&mut self) {
        self.tray.roll_all();
    }

    /// Clear all dice from the tray
    #[wasm_bindgen]
    pub fn clear(&mut self) {
        self.tray.clear();
    }

    ///Gets the tray data used to update the tray.
    #[wasm_bindgen]
    pub fn get_tray_data(&self) -> Result<String, JsValue> {
        let tray_data = TrayData::from_tray(&self.tray);
        serde_json::to_string(&tray_data).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
