use std::string;

use rust_dice::die_allocator::{Allocator, DieSummary};

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
    pub fn create_die(&mut self, sides: u32) -> Result<String, JsValue> {
        // Validate input
        if sides == 0 {
            return Err(JsValue::from_str("Die must have at least 1 side"));
        }
        if sides > 1000 {
            return Err(JsValue::from_str("Die cannot have more than 1,000 sides"));
        }
        
        // Log for debugging
        web_sys::console::log_1(&format!("Creating die with {} sides", sides).into());
        
        let die_summary = self.app_allocator.create_die(sides, None, None, 50)?;
        serde_json::to_string(&die_summary).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn get_dice_data(&self) -> Result<String, JsValue> {
        let dice_data = self.app_allocator.get_dice_summary();
        serde_json::to_string(&dice_data).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Roll all dice in the tray
    #[wasm_bindgen]
    pub fn roll_tray(&mut self, tray_id: usize) -> Result<String, JsValue> {
        let tray_summary = self.app_allocator.roll_tray(tray_id);
        serde_json::to_string(&tray_summary).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Clear all dice from the tray
    #[wasm_bindgen]
    pub fn clear_tray(&mut self, tray_id: usize) -> Result<String, JsValue> {
        let tray_summary = self.app_allocator.clear_readers_from_tray(tray_id)?;
        serde_json::to_string(&tray_summary).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    ///Gets the tray data used to update the tray.
    #[wasm_bindgen]
    pub fn get_tray_data(&self, tray_id: usize) -> Result<String, JsValue> {
        let tray_summary = self.app_allocator.get_tray_summary(tray_id)?;
        serde_json::to_string(&tray_summary).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
