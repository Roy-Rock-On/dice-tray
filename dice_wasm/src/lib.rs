use rust_dice::die_allocator::Allocator;

use wasm_bindgen::prelude::*;
use serde_json;
use web_sys;

#[wasm_bindgen]
pub fn greet(msg: &str) -> String{
    format!("{}", msg)
}

#[wasm_bindgen]
pub struct DiceAllocatorHandle {
    app_allocator: Allocator,
}

#[wasm_bindgen]
impl DiceAllocatorHandle {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            app_allocator: Allocator::new()
        }
    }

    /// Add a die to the tray
    #[wasm_bindgen]
    pub fn create_die(&mut self, sides: u32) -> Result<(), JsValue> {
        // Validate input
        if sides == 0 {
            return Err(JsValue::from_str("Die must have at least 1 side"));
        }
        if sides > 1000 {
            return Err(JsValue::from_str("Die cannot have more than 1,000 sides"));
        }
        
        // Log for debugging
        web_sys::console::log_1(&format!("Creating die with {} sides", sides).into());
        
        self.app_allocator.create_die(sides, None, None, 50);
        
        Ok(())
    }

    #[wasm_bindgen]
    pub fn get_dice_data(&self) -> Result<String, JsValue> {
        let dice_data = self.app_allocator.get_dice_summary();
        serde_json::to_string(&dice_data).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Roll all dice in the tray
    #[wasm_bindgen]
    pub fn roll_all(&mut self, tray_id: usize) -> Result<(), JsValue> {
        self.app_allocator.roll_tray();
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
