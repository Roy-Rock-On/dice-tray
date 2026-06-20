use js_sys::{JSON, Number};
use rust_dice::{die_allocator::Allocator, die_data::DieState, die_data::DieSort, die_tray::TraySummary};

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
    ///Constructor for the overall appHandle.
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

    /* DICE STUFF */
    /// Add a die to the dice bag
    #[wasm_bindgen]
    pub fn create_die(&mut self, sides: u32,  seed: u64, label: String, var: u32,) -> Result<JsValue, JsValue> {
        // Validate input
        if sides == 0 {
            return Err(JsValue::from_str("Die must have at least 1 side"));
        }
        if sides > 1000 {
            return Err(JsValue::from_str("Die cannot have more than 1,000 sides"));
        }
        
        // Log for debugging
        web_sys::console::log_1(&format!("Creating die with {} sides", sides).into());
        
        let die_summary = match self.app_allocator.create_die(sides, Some(seed), Some(label), var){
            Ok(summary) => summary,
            Err(e) => {
                let error_str = format!("Failed to create die. Error {}", e);
                return Err(JsValue::from_str(&error_str));
            }
        };
        serde_wasm_bindgen::to_value(&die_summary).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn destroy_dice(&mut self, ids: Vec<usize>) -> Result<JsValue, JsValue> {
        let die_summary = match self.app_allocator.destroy_dice(ids){
            Ok(summary) => summary,
            Err(e) => {
                let error_str = format!("Failed to create die. Error {}", e);
                return Err(JsValue::from_str(&error_str));
            }
        };
        serde_wasm_bindgen::to_value(&die_summary).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    ///rolls a die in the dice bag directly.
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

    ///Gets the dice state of all dice in the bag.
    #[wasm_bindgen]
    pub fn get_dice_state(&self, sort_mode: String) -> Result<JsValue, JsValue> {
        let sort = match sort_mode.trim(){
            "result" => DieSort::CurrentFace,
            "face" => DieSort::FaceCount,
            _ => DieSort::FaceCount
        };

        let dice_state = self.app_allocator.get_dice_state(Some(sort));
        web_sys::console::log_1(&format!("Found die stare {:?} sides", dice_state).into());
        serde_wasm_bindgen::to_value(&dice_state).map_err(|e| JsValue::from_str(&e.to_string()))
    }


    /* TRAY STUFF */
    ///Creates a new dice tray with the given name.
    #[wasm_bindgen]
    pub fn new_tray(&mut self, tray_id: String) -> Result<JsValue, JsValue>{       
        let tray_summary = match self.app_allocator.create_tray(tray_id){
            Ok(summary) => summary,
            Err(e) => return Err(JsValue::from_str(&e.to_string())) 
        };
        serde_wasm_bindgen::to_value(&tray_summary).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn destroy_tray(&mut self, tray_id: String) -> Result<(), JsValue>{
        match self.app_allocator.destroy_tray(&tray_id){
            Ok(_) => Ok(()),
            Err(e) => return Err(JsValue::from_str(&e.to_string()))
        }
    }

    ///Adds and rolls die readers to a tray.
    #[wasm_bindgen]
    pub fn roll_to_tray(&mut self, tray_id: String, die_id: usize, die_count: u32) -> Result<JsValue, JsValue>{
        for _ in 0..die_count{
            match self.app_allocator.add_die_reader(true, die_id, &Some(tray_id.clone())){
                Ok(_) => (),
                Err(e) => return Err(JsValue::from_str(&e.to_string()))
            };
        }

        match self.app_allocator.get_tray_summary(&tray_id, None){
            Ok(summary) => serde_wasm_bindgen::to_value(&summary).map_err(|e| JsValue::from_str(&e.to_string())),
            Err(e) => return Err(JsValue::from_str(&e.to_string()))
        }
    }

    #[wasm_bindgen]
    pub fn roll_in_tray(&mut self, tray_id: String, reader_ids: Vec<usize>, sort_mode: String) -> Result<JsValue, JsValue>{
        let die_sort = match sort_mode.trim(){
            "result" => DieSort::CurrentFace,
            "faces" => DieSort::FaceCount,
            _ => DieSort::FaceCount
        };
        
        match self.app_allocator.roll_at(Some(&tray_id), &reader_ids){
            Ok(()) => (),
            Err(e) => return Err(JsValue::from_str(&e.to_string()))
        };
        
        match self.app_allocator.get_tray_summary(&tray_id, Some(die_sort)){
            Ok(summary) => serde_wasm_bindgen::to_value(&summary).map_err(|e| JsValue::from_str(&e.to_string())),
            Err(e) => return Err(JsValue::from_str(&e.to_string()))
        }
    }

    /// Clear select dice readers from a tray.
    #[wasm_bindgen]
    pub fn clear_tray_readers(&mut self, reader_ids: Vec<usize>, tray_id: String) -> Result<JsValue, JsValue> {
        let tray_summary = match self.app_allocator.move_reader(false, &tray_id, &reader_ids, None){
            Ok(summary) => summary,
            Err(e) => return Err(JsValue::from_str(&e.to_string()))
        };
        serde_wasm_bindgen::to_value(&tray_summary).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    ///Gets the tray data used to update the tray.
    #[wasm_bindgen]
    pub fn get_tray_summary(&self, tray_id: String, sort_mode: String) -> Result<JsValue, JsValue> {
        let die_sort = match sort_mode.trim(){
            "result" => DieSort::CurrentFace,
            "faces" => DieSort::FaceCount,
            _ => DieSort::FaceCount
        };

        let tray_summary = match self.app_allocator.get_tray_summary(&tray_id, Some(die_sort)){
            Ok(summary) => summary,
            Err(e) => return Err(JsValue::from_str(&e.to_string()))
        };
        serde_wasm_bindgen::to_value(&tray_summary).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn get_all_tray_summaries(&self) -> Result<JsValue, JsValue> {
        let summaries = self.app_allocator.get_all_tray_summaries();
        serde_wasm_bindgen::to_value(&summaries).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
