use serde::{Deserialize, Serialize};

use crate::die::{DiceDataList, Die, DieData, RollLog, build_die};
use crate::die_reader::DieReader;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::{Display, Formatter};

//constants used to sutomaticaly weight the dice. 
const STD_WEIGHT: u32 = 100;

///Allocates integer IDs and reuses freed IDs in ascending order.
struct IdGenerator {
    next_id: usize,
    free_ids: BinaryHeap<Reverse<usize>>,
    free_set: HashSet<usize>,
}

impl IdGenerator {
    fn new() -> Self {
        Self {
            next_id: 0,
            free_ids: BinaryHeap::new(),
            free_set: HashSet::new(),
        }
    }

    fn alloc(&mut self) -> usize {
        if let Some(Reverse(id)) = self.free_ids.pop() {
            self.free_set.remove(&id);
            id
        } else {
            let return_id = self.next_id;
            self.next_id += 1;
            return_id
        }
    }

    fn free(&mut self, id: usize) {
        if id < self.next_id && self.free_set.insert(id) {
            self.free_ids.push(Reverse(id));
        }
    }
}

pub struct Allocator{
    tray_id_gen: IdGenerator,
    die_id_gen: IdGenerator,
    reader_id_gen: IdGenerator,
    dice: HashMap<usize, Die>,
    readers: HashMap<usize, DieReader>,
    trays: HashMap<usize, DieTray>
}

impl Allocator{
    ///Makes a new dice allocator for all your dice allocating needs.
    pub fn new() -> Self{
        Self { 
            tray_id_gen: IdGenerator::new(),
            die_id_gen: IdGenerator::new(),
            reader_id_gen: IdGenerator::new(),
            dice: HashMap::new(),
            readers: HashMap::new(),
            trays: HashMap::new() 
        }
    }

    ///Makes a new tray to track and sort dice being rolled.
    pub fn create_tray(&mut self, label: Option<String>){
        let new_tray_id = self.tray_id_gen.alloc();
        self.trays.insert(new_tray_id, DieTray::new(new_tray_id, label));        
    }

    ///Creates a new die. Does not add the dice to any tray.
    pub fn create_die(&mut self, faces: u32, seed: Option<u64>, label: Option<String>, varience: u32) -> Result<(), String>{
        let new_die_id = self.die_id_gen.alloc();
        
        let new_seed = match seed {
            Some(n) => n,
            None => {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                now.as_secs()
            }
        };

        let new_die = build_die(new_die_id, label, faces, new_seed, STD_WEIGHT, varience)?;
        self.dice.insert(new_die_id, new_die);
        Ok(())
    }

    ///Creates a new die. 
    ///If the die data contains a current_tray, attempts to move the die to a tray with matching ID.
    ///If unable (i.e. tray dosen't exist) returns clears the die's current_tray.  
    pub fn create_die_from_data(&mut self, die_data: DieData) -> Result<(), String> {
        let new_die_id = self.die_id_gen.alloc();
        let mut new_die = Die::from_data(die_data, new_die_id);
        new_die.roll();

        self.dice.insert(new_die_id, new_die);
        Ok(())
    }

    ///Creates new dice form a given DiceDataList. Uses new_die_from_data() internally.
    pub fn create_dice_from_list(&mut self, dice_data: DiceDataList) -> Result<(), String>{
        for data in dice_data.dice_data_vec {
            self.create_die_from_data(data)?; // now valid
        }
        Ok(())
    }

    pub fn destroy_dice(&mut self, targets: &DiceTargets) -> Result<(), String>{
        match targets{
            DiceTargets::All => {
                let die_ids: Vec<usize> = self.dice.keys().copied().collect();
                die_ids.into_iter().for_each(|id| self.die_id_gen.free(id));

                let reader_ids: Vec<usize> = self.readers.keys().copied().collect();
                reader_ids.into_iter().for_each(|id| self.reader_id_gen.free(id));

                self.dice.clear();
                self.readers.clear();
                self.trays.values_mut().for_each(|t| t.clear_readers());
            }
            DiceTargets::Index(indicies) =>{
                for i in indicies {
                    if self.dice.contains_key(&i){
                        self.prune_readers_for_die(*i);
                        self.dice.remove(&i);
                        self.die_id_gen.free(*i);
                    }
                }
            }
            DiceTargets::Label(label) => {
                let matching_dice = self.get_indices_by_label(&label);
                for i in matching_dice {
                    if self.dice.contains_key(&i){
                        self.prune_readers_for_die(i);
                        self.dice.remove(&i);
                        self.die_id_gen.free(i);
                    }
                }
            }
        }

        Ok(())
    }

    ///Prunes all readers that point to a specific die from the reader store and all trays.
    fn prune_readers_for_die(&mut self, die_id: usize) {
        let ids_to_remove: Vec<usize> = self.readers
            .iter()
            .filter(|(_, reader)| reader.get_die_id() == die_id)
            .map(|(id, _)| *id)
            .collect();

        for id in &ids_to_remove {
            self.readers.remove(id);
            self.reader_id_gen.free(*id);
        }

        self.trays.values_mut().for_each(|tray| tray.remove_readers_in(&ids_to_remove));
    }

    ///Removes a tray, prunes its readers from the reader store, and frees the tray ID.
    pub fn destroy_tray(&mut self, tray_id: usize) -> Result<(), String> {
        let tray = self.trays.remove(&tray_id)
            .ok_or_else(|| format!("No tray found with ID: {}", tray_id))?;

        for reader_id in tray.reader_ids {
            self.readers.remove(&reader_id);
            self.reader_id_gen.free(reader_id);
        }

        self.tray_id_gen.free(tray_id);
        Ok(())
    }

    ///Adds a new reader for an existing die to a target tray.
    ///Returns the reader ID on success.
    pub fn add_die_reader(&mut self, die_id: usize, tray_id: usize) -> Result<usize, String> {
        if !self.trays.contains_key(&tray_id) {
            return Err(format!("No tray found with ID: {}", tray_id));
        }

        let die = self.dice.get(&die_id)
            .ok_or_else(|| format!("No die found with ID: {}", die_id))?;

        let reader_id = self.reader_id_gen.alloc();
        let reader = DieReader::new(reader_id, die, tray_id);
        self.readers.insert(reader_id, reader);

        self.trays
            .get_mut(&tray_id)
            .ok_or_else(|| format!("No tray found with ID: {}", tray_id))?
            .add_reader(reader_id);
        Ok(reader_id)
    }

    ///Moves a reader to a different tray.
    ///tray_id takes an option. None will remove the reader from its current tray and reader store.
    pub fn move_reader(&mut self, reader_id: usize, new_tray_id: Option<usize>) -> Result<(), String> {
        if !self.readers.contains_key(&reader_id) {
            return Err(format!("No reader found with ID: {}", reader_id));
        }

        let old_tray_id = self.readers[&reader_id].get_tray_id();

        if let Some(tray) = self.trays.get_mut(&old_tray_id) {
            tray.remove_reader(reader_id);
        }

        match new_tray_id {
            None => {
                self.readers.remove(&reader_id);
                self.reader_id_gen.free(reader_id);
            }
            Some(id) => {
                self.readers
                    .get_mut(&reader_id)
                    .ok_or_else(|| format!("No reader found with ID: {}", reader_id))?
                    .set_tray(id);
                self.trays
                    .get_mut(&id)
                    .ok_or_else(|| format!("No tray found with ID: {}", id))?
                    .add_reader(reader_id);
            }
        }

        Ok(())
    }

    fn get_indices_by_label(&self, label: &str) -> Vec<usize>{
        let mut matching_ids = Vec::new();

        for die in self.dice.values() {
            if die.get_label() == label {
                matching_ids.push(die.get_id());
            }
        }

        matching_ids
    }

    ///Rolls the die with the given ID in place and returns a roll log.
    ///If the die has been assinged to a tray the tray will be updated.
    pub fn roll_die_reader(&mut self, reader_id: usize) -> Result<RollLog, String> {
        if let Some(selected_reader) = self.readers.get_mut(&reader_id){
            return Ok(selected_reader.roll(&mut self.dice)?);
        }
        else {
            Err(format!("No die reader found with given ID: {} Cannot roll.", reader_id))
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

    ///Sorts a tray by its held readers in the given ordering and returns the tray summary.
    pub fn sort_tray(&mut self, tray_id: usize, order: Ordering) -> Result<TraySummary<'_>, String> {
        let tray = self.trays.get_mut(&tray_id)
            .ok_or_else(|| format!("No tray found with ID: {}", tray_id))?;

        tray.sort(&self.readers, order)?;

        let tray_label = tray.label.clone();
        let tray_dice = tray.build_summary(&self.readers)?;
        Ok(TraySummary::new(tray_id, tray_label, tray_dice))
    }

    ///Returns readers currently in the requested tray in stored order.
    pub fn get_tray_summary(&self, tray_id: usize) -> Result<Vec<&DieReader>, String> {
        let tray = self.trays.get(&tray_id)
            .ok_or_else(|| format!("No tray found with ID: {}", tray_id))?;

        tray.build_summary(&self.readers)
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
    pub fn print_tray(&self, tray_id: usize) -> Result<(), String> {
        let tray = self.trays.get(&tray_id)
            .ok_or_else(|| format!("No tray found with ID: {}", tray_id))?;

        println!("Found tray with ID: {}", tray_id);
        println!("---{}---", tray.label);

        for reader_id in tray.reader_ids.iter(){
            if let Some(reader) = self.readers.get(reader_id) {
                let die_id = reader.get_die_id();
                if self.dice.contains_key(&die_id){
                    println!("{}", self.dice[&die_id]);
                }
                else{
                    return Err(format!("No dice found with ID: {}", die_id));
                }
            }
            else{
                return Err(format!("No reader found with ID: {}", reader_id));
            }
        }

        Ok(())
    }
}

pub struct DieTray{
    id: usize,
    label: String,
    reader_ids: Vec<usize>
}

impl DieTray{
    pub fn new(tray_id: usize, tray_name: Option<String>) -> Self{
        let tray_label = match tray_name{
            Some(s) => s,
            None =>{
                format!("Tray {}", tray_id.to_string())
            }
        };
        
        DieTray { 
            id: tray_id,
            label: tray_label,
            reader_ids: Vec::new()
        }
    }

    fn sort(
        &mut self,
        readers: &HashMap<usize, DieReader>,
        order: Ordering,
    ) -> Result<(), String> {
        let mut indexed: Vec<(usize, &DieReader)> = self.reader_ids
            .iter()
            .enumerate()
            .map(|(index, reader_id)| {
                let reader = readers.get(reader_id)
                    .ok_or_else(|| format!("Reader {} not found", reader_id))?;
                Ok((index, reader))
            })
            .collect::<Result<Vec<(usize, &DieReader)>, String>>()?;

        match order {
            Ordering::Less => indexed.sort_by(|a, b| a.1.cmp(&b.1)),
            Ordering::Greater => indexed.sort_by(|a, b| b.1.cmp(&a.1)),
            Ordering::Equal => {}
        }

        self.reader_ids = indexed
            .iter()
            .map(|(index, _)| self.reader_ids[*index])
            .collect();

        Ok(())
    }

    fn build_summary<'a>(
        &'a self,
        readers: &'a HashMap<usize, DieReader>,
    ) -> Result<Vec<&'a DieReader>, String> {
        self.reader_ids
            .iter()
            .map(|reader_id| {
                let reader = readers
                    .get(reader_id)
                    .ok_or_else(|| format!("Reader {} not found", reader_id))?;
                Ok(reader)
            })
            .collect()
    }

    ///Returns the die IDs referenced by readers in this tray.
    pub fn get_reader_die_ids(&self, readers: &HashMap<usize, DieReader>) -> Vec<usize>{
        self.reader_ids
            .iter()
            .filter_map(|reader_id| {
                readers.get(reader_id).map(|r| r.get_die_id())
            })
            .collect()
    }

    fn add_reader(&mut self, reader_id: usize){
        self.reader_ids.push(reader_id);
    }

    fn remove_reader(&mut self, reader_id: usize) {
        self.reader_ids.retain(|id| *id != reader_id);
    }

    fn remove_readers_in(&mut self, ids: &[usize]) {
        self.reader_ids.retain(|id| !ids.contains(id));
    }

    fn clear_readers(&mut self){
        self.reader_ids.clear();
    }
}

impl Display for DieTray{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tray ID = {}, Tray Label = {}, Count of dice in tray = {}",
            self.id,
            self.label,
            self.reader_ids.len()
        )
    }
}

pub struct TraySummary<'a>{
    pub tray_id: usize,
    pub tray_label: String,
    pub tray_dice: Vec<&'a DieReader>
}

impl<'a> TraySummary<'a>{
    pub fn new(id: usize, label: String, dice: Vec<&'a DieReader>) -> Self {
        TraySummary { 
            tray_id: id, 
            tray_label: label,
            tray_dice: dice
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum DiceTargets {
    All,
    Index(Vec<usize>),
    Label(String),
}

impl Display for DiceTargets{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DiceTargets::All => write!(f, "all"),
            DiceTargets::Index(indices) => write!(f, "indices={:?}", indices),
            DiceTargets::Label(label) => write!(f, "label=\"{}\"", label),
        }
    }
}

#[cfg(test)]
#[test]
fn test_new_tray(){
    let mut allocator = Allocator::new();
    allocator.create_tray(Some("Best Tray".to_string()));
    allocator.create_tray(Some("Worst Tray".to_string()));
    allocator.create_tray(None);

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
    allocator.create_tray(Some("THE TRAY".to_string()));
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
fn test_dice_to_tray() -> Result<(), String>{
    let mut allocator = test_build_allocator_from_file().unwrap();
    allocator.print_dice();
    allocator.add_die_reader(2, 0)?;
    allocator.add_die_reader(1, 0)?;
    allocator.print_tray(0)?;
    
    allocator.create_tray(Some("The tray to end all trays".to_string()));
    let reader_5 = allocator.add_die_reader(5, 1)?;
    allocator.print_tray(1)?;

    allocator.move_reader(reader_5, Some(0))?;
    allocator.print_tray(0)?;

    allocator.move_reader(reader_5, None)?;

    allocator.print_dice();

    Ok(())
}

#[test]
fn test_die_tray_sort() -> Result<(), String> {
    let mut allocator = Allocator::new();
    allocator.create_tray(Some("Sort Tray".to_string()));

    allocator.create_die(20, Some(11), Some("d20".to_string()), 10)?;
    allocator.create_die(4, Some(22), Some("d4".to_string()), 10)?;
    allocator.create_die(12, Some(33), Some("d12".to_string()), 10)?;
    allocator.create_die(6, Some(44), Some("d6".to_string()), 10)?;

    allocator.add_die_reader(0, 0)?;
    allocator.add_die_reader(1, 0)?;
    allocator.add_die_reader(2, 0)?;
    allocator.add_die_reader(3, 0)?;

    let sorted = {
        let readers = &allocator.readers;
        let tray = allocator.trays.get_mut(&0).ok_or_else(|| "Tray not found".to_string())?;
        tray.sort(readers, Ordering::Less)?;
        tray.build_summary(readers)?
    };

    let sorted_ids = sorted.iter().map(|reader| reader.get_die_id()).collect::<Vec<usize>>();
    assert_eq!(sorted_ids, vec![1, 3, 2, 0]);

    Ok(())
}

#[test]
fn test_allocator_sort_tray() -> Result<(), String> {
    let mut allocator = Allocator::new();
    allocator.create_tray(Some("Allocator Sort Tray".to_string()));

    allocator.create_die(20, Some(101), Some("d20".to_string()), 25)?;
    allocator.create_die(4, Some(202), Some("d4".to_string()), 25)?;
    allocator.create_die(12, Some(303), Some("d12".to_string()), 25)?;
    allocator.create_die(6, Some(404), Some("d6".to_string()), 25)?;

    allocator.add_die_reader(0, 0)?;
    allocator.add_die_reader(1, 0)?;
    allocator.add_die_reader(2, 0)?;
    allocator.add_die_reader(3, 0)?;

    let sorted_asc = allocator.sort_tray(0, Ordering::Less)?;
    let asc_ids = sorted_asc.tray_dice.iter().map(|reader| reader.get_die_id()).collect::<Vec<usize>>();
    assert_eq!(asc_ids, vec![1, 3, 2, 0]);

    let sorted_desc = allocator.sort_tray(0, Ordering::Greater)?;
    let desc_ids = sorted_desc.tray_dice.iter().map(|reader| reader.get_die_id()).collect::<Vec<usize>>();
    assert_eq!(desc_ids, vec![0, 2, 3, 1]);

    let tray_ids = allocator
        .trays
        .get(&0)
        .ok_or_else(|| "Tray not found".to_string())?
        .get_reader_die_ids(&allocator.readers);
    assert_eq!(tray_ids, desc_ids);

    Ok(())
}

#[test]
fn test_die_id_reuse_after_remove() -> Result<(), String> {
    let mut allocator = Allocator::new();

    allocator.create_die(6, Some(1), Some("a".to_string()), 25)?;
    allocator.create_die(8, Some(2), Some("b".to_string()), 25)?;
    allocator.create_die(10, Some(3), Some("c".to_string()), 25)?;

    allocator.destroy_dice(&DiceTargets::Index(vec![1]))?;
    allocator.create_die(12, Some(4), Some("d".to_string()), 25)?;

    assert!(allocator.dice.contains_key(&1));
    assert_eq!(allocator.dice.len(), 3);
    Ok(())
}

#[test]
fn test_reader_id_reuse_after_remove() -> Result<(), String> {
    let mut allocator = Allocator::new();
    allocator.create_tray(Some("t0".to_string()));

    allocator.create_die(6, Some(1), Some("d0".to_string()), 25)?;
    allocator.create_die(6, Some(2), Some("d1".to_string()), 25)?;

    let reader_zero = allocator.add_die_reader(0, 0)?;
    let _reader_one = allocator.add_die_reader(1, 0)?;

    allocator.move_reader(reader_zero, None)?;
    let reused_reader = allocator.add_die_reader(0, 0)?;

    assert_eq!(reused_reader, reader_zero);
    Ok(())
}

#[test]
fn test_tray_id_reuse_after_remove() -> Result<(), String> {
    let mut allocator = Allocator::new();
    allocator.create_tray(Some("t0".to_string()));
    allocator.create_tray(Some("t1".to_string()));

    allocator.destroy_tray(0)?;
    allocator.create_tray(Some("t2".to_string()));

    assert!(allocator.trays.contains_key(&0));
    assert!(allocator.trays.contains_key(&1));
    assert_eq!(allocator.trays.len(), 2);
    Ok(())
}


