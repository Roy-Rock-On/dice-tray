use serde::{Deserialize, Serialize};

use crate::die::{self, DiceDataList, Die, DieData, DieResult, build_die};
use crate::die_targets::DiceTargets;
use crate::die_tray::{DieTray, MoveSummary, TraySummary};
use crate::id_generator::IdGenerator;

use std::cmp::{Ordering};
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Error, bail};

//constants used to sutomaticaly weight the dice. 
const STD_WEIGHT: u32 = 100;

pub struct Allocator{
    die_id_gen: IdGenerator,
    dice: HashMap<usize, Die>,
    trays: HashMap<String, DieTray>,
    target_tray_label: String
}

impl Allocator{
    ///Makes a new dice allocator for all your dice allocating needs.
    pub fn new() -> Self{
        Self { 
            die_id_gen: IdGenerator::new(),
            dice: HashMap::new(),
            trays: HashMap::new(),
            target_tray_label: "Main".to_string()
        }
    }

    ///Gets the target tray label. 
    fn get_target_tray(&self) -> &str {
        &self.target_tray_label
    }


    pub fn set_target_tray(&mut self, new_target_tray: String) -> anyhow::Result<()>{
        if self.trays.contains_key(&new_target_tray){
            self.target_tray_label = new_target_tray;  
            Ok(())
        }
        else{
            bail!("No tray in allocator with label: {} Maybe you need to create the tray first.", &new_target_tray)
        }
    }

    ///Makes a new tray to track and sort dice being rolled. Returns a summary of the new tray (which will be empty).
    pub fn create_tray(&mut self, label: String) -> anyhow::Result<TraySummary>{
        if self.trays.contains_key(&label){
            bail!("Allocator already contains a tray with label: {} All tray labels must bey unique.", label)
        }

        let new_tray = DieTray::new(label.clone());
        self.trays.insert(label.clone(), new_tray);
        Ok(self.trays.get(&label).unwrap().build_summary())        
    }

    ///Creates a new die. Does not add the dice to any tray.
    pub fn create_die(&mut self, faces: u32, seed: Option<u64>, label: Option<String>, varience: u32) -> anyhow::Result<DieSummary>{
        let new_die_id = self.die_id_gen.allocate();
        
        let new_seed = match seed {
            Some(n) => n,
            None => {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                now.as_secs()
            }
        };

        let new_die = build_die(new_die_id, label, faces, new_seed, STD_WEIGHT, varience)?;
        let summary = DieSummary::from_die(&new_die);
        self.dice.insert(new_die_id, new_die);
        Ok(summary)
    }

    ///Creates a new die. 
    ///If the die data contains a current_tray, attempts to move the die to a tray with matching ID.
    ///If unable (i.e. tray dosen't exist) returns clears the die's current_tray.  
    pub fn create_die_from_data(&mut self, die_data: DieData) -> anyhow::Result<()> {
        let new_die_id = self.die_id_gen.allocate();
        let mut new_die = Die::from_data(die_data, new_die_id);
        new_die.roll();

        self.dice.insert(new_die_id, new_die);
        Ok(())
    }

    ///Creates new dice form a given DiceDataList. Uses new_die_from_data() internally.
    pub fn create_dice_from_list(&mut self, dice_data: DiceDataList) -> anyhow::Result<()>{
        for data in dice_data.dice_data_vec {
            self.create_die_from_data(data)?; // now valid
        }
        Ok(())
    }

    ///Destroys dice in the dice bag, clearing any attached dice readers.
    pub fn destroy_dice(&mut self, targets: Vec<usize>) -> anyhow::Result<DiceSummary>{
        for id in targets{
            if self.dice.contains_key(&id){
                self.prune_readers_for_die(id);
                self.die_id_gen.free(id);
                self.dice.remove(&id);
            }
        }

        Ok(self.get_dice_summaries())
    }

    ///Prunes all readers that point to a specific die from the reader store and all trays.
    fn prune_readers_for_die(&mut self, die_id: usize) {
        self.trays.values_mut().for_each(|tray| tray.remove_readers_by_die_id(die_id));
    }

    ///Removes a tray, prunes its readers from the reader store, and frees the tray ID.
    pub fn destroy_tray(&mut self, tray_label: String) -> anyhow::Result<()> {
        let mut tray = match self.trays.remove(&tray_label){
            Some(tray) => tray,
            None => bail!("Cannot destroy tray. No tray found with given label: {}", tray_label)
        };

        tray.clear_readers();
        Ok(())
    }

    ///Adds a new reader for an existing die to a target tray.
    ///Returns a TraySummary of the given tray.
    pub fn add_die_reader(&mut self, die_id: usize, tray_label: &Option<String>) -> anyhow::Result<usize> {
        
        let tray_label = match tray_label{
            Some(label) => label,
            None => &self.target_tray_label
        };
        
        if !self.trays.contains_key(tray_label) {
            bail!("No tray found with label: {}", tray_label);
        }

        let die = match self.dice.get(&die_id){
            Some(die) => die,
            None => bail!("No die found with ID: {} | Cannot build a die reader.", die_id)
        };

        let tray = match self.trays.get_mut(tray_label){
            Some(tray) => tray,
            None => bail!("No tray found with Label: {} | Cannot add die reader to tray.", tray_label)
        };

        let reader_id = tray.add_reader(die);    

        Ok(reader_id)
    }

    ///Removes all the die readers from a given tray.
    pub fn clear_readers_from_tray(&mut self, tray_label : String) -> anyhow::Result<()> {
        if let Some(tray) = self.trays.get_mut(&tray_label){
            tray.clear_readers();
            Ok(())
        }
        else{
            bail!("No tray found with ID: {}", tray_label);
        }
    }

    ///Moves a reader to a different tray.
    ///tray_id takes an option. None will remove the reader from its current tray and reader store.
    pub fn move_reader(&mut self, from_tray: String, reader_ids: Vec<usize>, to_tray: Option<String>) -> anyhow::Result<MoveSummary> {
        let removal_tray = match self.trays.get_mut(&from_tray){
            Some(tray) => tray,
            None => bail!("No tray found with ID: {}", from_tray)
        };
        
        let die_ids = match removal_tray.remove_readers(&reader_ids){
            Some(ids) => ids,
            None => bail!("No readers found with IDs {:?} in tray {}", reader_ids, removal_tray.get_label())
        };

        let removal_summary = removal_tray.build_summary();

        match to_tray{
            Some(to_id) => {
                let to_tray = match self.trays.get_mut(&to_id){
                    Some(tray) => tray,
                    None => bail!("No tray found with ID: {}", from_tray)
                };

                for id in die_ids {
                    let die = match self.dice.get(&id){
                        Some(die) => die,
                        None => bail!("No die found with ID: {}", id)
                    };
                    to_tray.add_reader(die);
                }

                let move_summary = to_tray.build_summary();
                Ok(MoveSummary::new(removal_summary, Some(move_summary)))
            },
            None => Ok(MoveSummary::new(removal_summary, None))
        }
    }

    ///Gets a list of die IDs given a DiceTarget enum.
    pub fn get_die_ids_from_targets(&self, targets: &DiceTargets) -> Option<Vec<usize>>{
        let mut matching_ids = HashSet::new();

        match targets{
            DiceTargets::All =>{
                for die in self.dice.values(){
                    matching_ids.insert(die.get_id());
                }
            },
            DiceTargets::Index(indecies) => {
                for index in indecies{
                    if matching_ids.contains(&index){
                        matching_ids.insert(*index);
                    }
                }
            },
            DiceTargets::Label(labels) => {
                for die in self.dice.values(){
                    if labels.contains(&die.get_label().to_string()){
                        matching_ids.insert(die.get_id());
                    }
                }
            },
            DiceTargets::None => ()
        }

        let matching_ids: Vec<usize> = matching_ids.into_iter().collect();

        if matching_ids.len() > 0{
            Some(matching_ids)
        }
        else{
            None
        }
    }

    ///Gets IDs of die reader in the tray based on the DiceTargets provided.
    pub fn get_reader_ids_by_targets(&self, tray_id : String, targets: DiceTargets) -> Result<Vec<usize>, String>{
        if let Some(tray) = self.trays.get(&tray_id){
            match tray.get_reader_ids_by_targets(&targets){
                Some(reder_ids) => Ok(reder_ids),
                None => Err(format!("No die readers found in tray ID = {} at targets {}", tray_id, targets))
            }    
        }
        else{
            Err(format!("No tray ID = {} found in application.", tray_id))
        }
    }

    ///Rolls the die with the given ID in place and returns a roll log.
    ///If the die has been assinged to a tray the tray will be updated.
    pub fn roll_at(&mut self, tray_id: &Option<String>, dice_targets: &[usize]) -> anyhow::Result<()> {
        let tray_label = match tray_id{
            Some(label) => label,
            None => &self.target_tray_label
        };

        if let Some(selected_tray) = self.trays.get_mut(tray_label){
            selected_tray.roll_at(dice_targets, &mut self.dice)?;
            Ok(())
        }
        else {
            bail!("No tray found with ID: {} Cannot roll.", tray_label)
        }
    }

    ///Iterates through all the dice in allocator and returns a dice data list for serilization.
    pub fn build_die_data_list(&self, new_file_name: Option<String>) -> DiceDataList {
        let file_name = match new_file_name{
            Some(name) => name,
            None => "DiceData".to_string()
        };

        let mut dice_data_list = DiceDataList::new(file_name);

        for die in self.dice.values(){
            dice_data_list.add_data(die.to_data());
        } 
        
        dice_data_list
    }

    ///Gets a refrence to all the Die structs in the allocator.
    pub fn get_dice(&self) -> Vec<&Die>{
        self.dice.values().collect()
    }

    ///Gets a summary of all the dice in the allocator.
    pub fn get_dice_summaries(&self) -> DiceSummary{
        DiceSummary::new(self.dice.values().map(|d| DieSummary::from_die(d)).collect())
    }

    ///Gets summaries for every tray.
    pub fn get_all_tray_summarys(&self) -> Vec<TraySummary>{
        self.trays.values().map(|tray| tray.build_summary()).collect()
    }

    ///Gets a tray summary for the given tray ID.
    pub fn get_tray_summary_at(&self, tray_id : &Option<String>) -> anyhow::Result<TraySummary>{
        let tray_id = match tray_id{
            Some(id) => id,
            None => &self.target_tray_label
        };
        
        if let Some(tray) = self.trays.get(tray_id){
            Ok(tray.build_summary())
        }
        else{
            bail!("No tray found with id {}", tray_id)
        }
    }

    pub fn roll_tray(&self, tray_id : String) -> anyhow::Result<TraySummary>{
        if let Some(tray) = self.trays.get(&tray_id){
            Ok(tray.build_summary())
        }
        else{
           bail!("No tray found with id {}", tray_id)
        }
    }

    ///Sorts a tray by its held readers in the given ordering and returns the tray summary.
    pub fn sort_tray(&mut self, tray_id: &str, order: Ordering) -> anyhow::Result<TraySummary> {
        let tray = match self.trays.get_mut(tray_id){
            Some(tray) => tray,
            None => bail!("No tray with given label {} found.", tray_id)
        };

        tray.sort(order);

        Ok(tray.build_summary())
    }

    ///Returns readers currently in the requested tray in stored order.
    pub fn get_tray_summary(&self, tray_id: String) -> Result<TraySummary, String> {
        let tray = self.trays.get(&tray_id)
            .ok_or_else(|| format!("No tray found with ID: {}", tray_id))?;

        Ok(tray.build_summary())
    }

    ///Returns a summary of all the dice in the Dice Bag.
    pub fn get_dice_summary(&self) -> DiceSummary  {
        let dice_summary = self.dice.values()
            .map(|d| DieSummary::from_die(d))
            .collect();

        DiceSummary { dice : dice_summary }
    }

    ///Prints a list of all dice in the allocator
    ///For use in CLI or debugging. 
    pub fn print_dice(&self){
        println!("---ALL DICE IN ALLOCATOR---");
        for die in self.dice.values(){
            println!{"{}", die};
        }
    }

    ///Prints out a list of dice in a given tray. 
    ///For use in CLI or debugging.
    pub fn print_tray(&self, tray_id: String) -> Result<(), String> {
        let tray = self.trays.get(&tray_id)
            .ok_or_else(|| format!("No tray found with ID: {}", tray_id))?;

        tray.build_summary().print();
        Ok(())
    }
}



#[derive(Serialize, Deserialize)]
pub struct DiceSummary{
    dice: Vec<DieSummary>
}

impl DiceSummary{
    pub fn new(die_summaries: Vec<DieSummary>) -> Self{
        DiceSummary{
            dice: die_summaries
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DieSummary{
    id: usize,
    label: String,
    faces: u32,
    current_face: u32,
    result: DieResult
}

impl DieSummary{
    pub fn from_die(die: &Die) -> Self{
        DieSummary { 
            id: die.get_id(),
            label: die.get_label().to_string(),
            faces: die.get_face_count(),
            current_face: die.get_current_face(),
            result: die.get_current_result().clone()
        }
    }

    pub fn print(&self){
        println!("Die Summary:");
        println!("id: {} | label: {} | faces: {} | current_face: {}", self.id, self.label, self.faces, self.current_face);
    }
}

#[cfg(test)]
#[test]
fn test_new_tray(){
    let mut allocator = Allocator::new();
    let _ = allocator.create_tray("Best Tray".to_string());
    let _ = allocator.create_tray("Worst Tray".to_string());
    let _ = allocator.create_tray("Another Tray".to_string());

    for t in allocator.trays.values(){
        println!("{}", t);
    }
}

#[cfg(test)]
fn test_build_allocator_from_file() -> Result<Allocator, String> {
    use std::fs;
    use std::path::PathBuf;
    use dotenv;

    dotenv::from_filename("rust_dice/src/.env").unwrap();

    let rel = PathBuf::from(std::env::var("DICE_DATA_PATH").unwrap());
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);
    let file_path = data_dir.join("die_test");
    let die_file = fs::read_to_string(&file_path).unwrap();

    let decoded_list: DiceDataList = serde_json::from_str(&die_file).unwrap();
    
    let mut allocator = Allocator::new();
    let _ = allocator.create_tray("THE TRAY".to_string());
    allocator.create_dice_from_list(decoded_list).unwrap();
    Ok(allocator)
}

#[test]
fn test_dice_from_list() -> Result<(), String>{
    let allocator = test_build_allocator_from_file().unwrap();
    allocator.print_dice();
    Ok(())
}

#[test]
fn test_die_tray_sort() -> anyhow::Result<()> {
    let mut allocator = Allocator::new();
    let _ = allocator.create_tray("Sort Tray".to_string());

    allocator.create_die(20, Some(11), Some("d20".to_string()), 10)?;
    allocator.create_die(4, Some(22), Some("d4".to_string()), 10)?;
    allocator.create_die(12, Some(33), Some("d12".to_string()), 10)?;
    allocator.create_die(6, Some(44), Some("d6".to_string()), 10)?;

    allocator.add_die_reader(0, &Some("Tray!".to_string()))?;
    allocator.add_die_reader(1, &Some("Tray!".to_string()))?;
    allocator.add_die_reader(2, &Some("Tray!".to_string()))?;
    allocator.add_die_reader(3, &Some("Tray!".to_string()))?;

    if let Ok(summary) = allocator.sort_tray("Tray!", Ordering::Greater){
        summary.print(); 
    }

    Ok(())
}


#[test]
fn test_die_id_reuse_after_remove() -> anyhow::Result<()> {
    let mut allocator = Allocator::new();

    allocator.create_die(6, Some(1), Some("a".to_string()), 25)?;
    allocator.create_die(8, Some(2), Some("b".to_string()), 25)?;
    allocator.create_die(10, Some(3), Some("c".to_string()), 25)?;

    allocator.destroy_dice(vec![1])?;
    allocator.create_die(12, Some(4), Some("d".to_string()), 25)?;

    assert!(allocator.dice.contains_key(&1));
    assert_eq!(allocator.dice.len(), 3);
    Ok(())
}



