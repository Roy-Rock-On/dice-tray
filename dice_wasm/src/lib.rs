use js_sys::{JSON, Number};
use rust_dice::{die_allocator::Allocator, die_tray::TraySummary};

use anyhow::{Result, bail};
use wasm_bindgen::prelude::*;
use std::format;
use web_sys;

#[wasm_bindgen]
pub struct DiceAllocatorHandle {
    app_allocator: Allocator,
}

#[wasm_bindgen]
impl DiceAllocatorHandle {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Self, JsValue>  {
        console_error_panic_hook::set_once();
        let app_allocator = match Allocator::new(){
            Ok(alloc) => alloc,
            Err(e) => {
                let error_str = format!("WASM Failed to create a die allocator handle. Error {}", e);
                return Err(JsValue::from_str(&error_str))
            } 
        };

        let alloc_handle = Self {
            app_allocator
        };

        Ok(alloc_handle)
    }

    /// Add a die to the tray
    #[wasm_bindgen]
    pub fn create_die(&mut self, sides: u32, seed: u64) -> Result<JsValue, JsValue> {
        // Validate input
        if sides == 0 {
            return Err(JsValue::from_str("Die must have at least 1 side"));
        }
        if sides > 1000 {
            return Err(JsValue::from_str("Die cannot have more than 1,000 sides"));
        }
        
        // Log for debugging
        web_sys::console::log_1(&format!("Creating die with {} sides", sides).into());
        
        let die_summary = match self.app_allocator.create_die(sides, Some(seed), None, 50){
            Ok(summary) => summary,
            Err(e) => {
                let error_str = format!("Failed to create die. Error {}", e);
                return Err(JsValue::from_str(&error_str));
            }
        };
        serde_wasm_bindgen::to_value(&die_summary).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn get_dice_data(&self) -> Result<JsValue, JsValue> {
        let dice_data = self.app_allocator.get_dice_summary();
        serde_wasm_bindgen::to_value(&dice_data).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Roll all dice in the tray
    #[wasm_bindgen]
    pub fn roll_tray(&mut self, tray_id: String) -> Result<JsValue, JsValue> {
        let tray_summary = match self.app_allocator.roll_tray(tray_id){
            Ok(summary) => summary,
            Err(e) => return Err(JsValue::from_str(&e.to_string())) 
        };
        serde_wasm_bindgen::to_value(&tray_summary).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    //rolls a die in the dice bag directly.
    #[wasm_bindgen]
    pub fn roll_die(&mut self, die_id: usize) -> Result<JsValue, JsValue> {
        // Log for debugging
        web_sys::console::log_1(&format!("Rolling die with ID = {}", die_id).into());
        match self.app_allocator.roll_die(die_id){
            Ok(summary) => return serde_wasm_bindgen::to_value(&summary)
                .map_err(|e| JsValue::from_str(&e.to_string())),
            Err(e) => return Err(JsValue::from_str(&e.to_string()))
        }
    }

    /// Clear all dice from the tray
    #[wasm_bindgen]
    pub fn clear_tray(&mut self, tray_id: String) -> Result<JsValue, JsValue> {
        let tray_summary = match self.app_allocator.clear_readers_from_tray(tray_id){
            Ok(summary) => summary,
            Err(e) => return Err(JsValue::from_str(&e.to_string()))
        };
        serde_wasm_bindgen::to_value(&tray_summary).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    ///Gets the tray data used to update the tray.
    #[wasm_bindgen]
    pub fn get_tray_data(&self, tray_id: String) -> Result<JsValue, JsValue> {
        let tray_summary = match self.app_allocator.get_tray_summary(&tray_id){
            Ok(summary) => summary,
            Err(e) => return Err(JsValue::from_str(&e.to_string()))
        };
        serde_wasm_bindgen::to_value(&tray_summary).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
