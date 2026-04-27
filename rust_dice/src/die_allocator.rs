use crate::die::{DiceDataList, Die, DieData, DieSummary, RollLog, build_die};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::Display;

use std::cmp::Ordering;

//constants used to sutomaticaly weight the dice. 
const STD_WEIGHT: u32 = 100;
const VAR_WEIGHT: u32 = 50;

//Id generator manages ids with internal mutability.
struct DieIdGenerator {
    next_die_id: usize
}

impl DieIdGenerator{
    fn new() -> Self{
        Self { next_die_id: 0 }
    }

    fn get_next_die_id(&mut self) -> usize{
        let return_id = self.next_die_id;
        self.next_die_id += 1;
        return_id
    }
}

struct TrayIDGenerator{
    next_tray_id: usize
}

impl TrayIDGenerator{
    fn new() -> Self{
        Self { next_tray_id: 0 }
    }

    fn get_next_tray_id(&mut self) -> usize{
        let return_id = self.next_tray_id;
        self.next_tray_id += 1;
        return_id
    }
}

pub struct Allocator{
    tray_id_gen: TrayIDGenerator,
    die_id_gen: DieIdGenerator,
    dice: HashMap<usize, Die>,
    trays: HashMap<usize, DieTray>
}

impl Allocator{
    ///Makes a new dice allocator for all your dice allocating needs.
    pub fn new() -> Self{
        Self { 
            tray_id_gen: TrayIDGenerator::new(),
            die_id_gen: DieIdGenerator::new(),
            dice: HashMap::new(),
            trays: HashMap::new() 
        }
    }

    ///Makes a new tray to track and sort dice being rolled.
    pub fn new_tray(&mut self, label: Option<String>){
        let new_tray_id = self.tray_id_gen.get_next_tray_id();
        self.trays.insert(new_tray_id, DieTray::new(new_tray_id, label));        
    }

    ///Creates a new die. Does not add the dice to any tray.
    pub fn new_die(&mut self, faces: u32, seed: Option<u64>, label: Option<String>) -> Result<(), String>{
        let new_die_id = self.die_id_gen.get_next_die_id();
        
        let new_seed = match seed {
            Some(n) => n,
            None => {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                now.as_secs()
            }
        };

        let new_die = build_die(new_die_id, label, faces, new_seed, STD_WEIGHT, VAR_WEIGHT)?;
        self.dice.insert(new_die_id, new_die);
        Ok(())
    }

    ///Creates a new die. 
    ///If the die data contains a current_tray, attempts to move the die to a tray with matching ID.
    ///If unable (i.e. tray dosen't exist) returns clears the die's current_tray.  
    pub fn new_die_from_data(&mut self, die_data: DieData) -> Result<(), String> {
        let new_die_id = self.die_id_gen.get_next_die_id();
        let mut new_die = Die::from_data(die_data, new_die_id);

        if let Some(id) = new_die.get_tray_id(){
            if let Some(tray) = self.trays.get_mut(&id){
                tray.add_die(new_die_id);
            }
            else{
                new_die.set_tray(None);
            }
        }

        self.dice.insert(new_die_id, new_die);
        Ok(())
    }

    ///Creates new dice form a given DiceDataList. Uses new_die_from_data() internally.
    pub fn new_dice_from_list(&mut self, dice_data: DiceDataList) -> Result<(), String>{
        for data in dice_data.dice_data_vec {
            self.new_die_from_data(data)?; // now valid
        }
        Ok(())
    }

    ///Moves the die with the given die_id to the tray with the given tray_id. 
    ///tray_id takes an option. None will result in the die being removed form all trays.
    ///This function updates both the dice's internal current_tray and the tray's dice ID list.
    pub fn move_die(&mut self, die_id: usize, tray_id: Option<usize>) -> Result<(), String>{
        let die = self.dice.get_mut(&die_id)
            .ok_or_else(|| format!("No die found with ID: {}", die_id))?;

        let die_id = die.get_id();

        if let Some(current_tray_id) = die.get_tray_id() {
            if let Some(tray) = self.trays.get_mut(&current_tray_id) {
                tray.remove_die(die_id)?;
            }
        }

        match tray_id{
            None => die.set_tray(None),
            Some(id) => {
                if let Some(tray) = self.trays.get_mut(&id){
                    die.set_tray(tray_id);
                    tray.add_die(die_id);
                }
                else{
                    return Err(format!("No tray found with ID: {}", id));
                }
            }
        }
        Ok(())
    }

    ///Rolls the die with the given ID in place and returns a roll log.
    ///If the die has been assinged to a tray the tray will be updated.
    pub fn roll_die(&mut self, die_id: usize) -> Result<RollLog, String> {
        if let Some(selected_die) = self.dice.get_mut(&die_id){
            return Ok(selected_die.roll());
        }
        else {
            Err(format!("No die found with given die ID: {} Cannot roll.", die_id))
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


    pub fn sort_tray(&mut self, tray_id: usize, order: Ordering) -> Result<Vec<DieSummary>, String> {
        let tray = self.trays.get_mut(&tray_id)
            .ok_or_else(|| format!("No tray found with ID: {}", tray_id))?;

        let tray_dice = tray.get_dice();
        let dice_summaries = tray_dice.into_iter()
            .map(|id|{self.dice[id].to_summary()})
            .collect();

        let sorted_dice = tray.sort(dice_summaries, order);
        Ok(sorted_dice)
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

        for die_id in tray.dice.iter(){
            if self.dice.contains_key(&die_id){
                println!("{}", self.dice[&die_id]);
            }
            else{
                return Err(format!("No dice found with ID: {}", die_id));
            }
        }

        Ok(())
    }
}

pub struct DieTray{
    id: usize,
    label: String,
    dice: Vec<usize>
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
            dice: Vec::new()
        }
    }

    fn sort(&mut self, mut dice_summaries: Vec<DieSummary>, order: Ordering) -> Vec<DieSummary> {
        match order {
            Ordering::Less => dice_summaries.sort(),
            Ordering::Greater => dice_summaries.sort_by(|a, b| b.cmp(a)),
            Ordering::Equal => {}
        }

        self.dice = dice_summaries.iter()
            .map(|die| die.get_id())
            .collect();

        dice_summaries
    }

    ///Returns a refrence to the dice_ids in the tray.
    fn get_dice(&self) -> &Vec<usize>{
        &self.dice
    }

    fn add_die(&mut self, die_id : usize){
        self.dice.push(die_id);
    }

    fn remove_die(&mut self, die_id: usize) -> Result<usize, String>{
        if let Some(pos) = self.dice.iter().position(|die| *die == die_id){
            return Ok(self.dice.remove(pos));
        }
        else{
            Err(format!("No die with ID: {} Found in Tray with ID: {}", die_id, self.id))
        }
    }


}

impl Display for DieTray{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tray ID = {}, Tray Label = {}, Count of dice in tray = {}",
            self.id,
            self.label,
            self.dice.len()
        )
    }
}

#[cfg(test)]
#[test]
fn test_new_tray(){
    let mut allocator = Allocator::new();
    allocator.new_tray(Some("Best Tray".to_string()));
    allocator.new_tray(Some("Worst Tray".to_string()));
    allocator.new_tray(None);

    for t in allocator.trays.values(){
        println!("{}", t);
    }
}

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
    allocator.new_tray(Some("THE TRAY".to_string()));
    allocator.new_dice_from_list(decoded_list).unwrap();
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
    allocator.move_die(2, Some(0))?;
    allocator.move_die(1, Some(0))?;
    allocator.print_tray(0)?;
    
    allocator.new_tray(Some("The tray to end all trays".to_string()));
    allocator.move_die(5, Some(1))?;
    allocator.print_tray(1)?;

    allocator.move_die(5, Some(0))?;
    allocator.print_tray(0)?;

    allocator.move_die(5, None)?;

    allocator.print_dice();

    Ok(())
}

#[test]
fn test_die_tray_sort() -> Result<(), String> {
    let mut allocator = Allocator::new();
    allocator.new_tray(Some("Sort Tray".to_string()));

    allocator.new_die(20, Some(11), Some("d20".to_string()))?;
    allocator.new_die(4, Some(22), Some("d4".to_string()))?;
    allocator.new_die(12, Some(33), Some("d12".to_string()))?;
    allocator.new_die(6, Some(44), Some("d6".to_string()))?;

    allocator.move_die(0, Some(0))?;
    allocator.move_die(1, Some(0))?;
    allocator.move_die(2, Some(0))?;
    allocator.move_die(3, Some(0))?;

    let summaries = {
        let tray = allocator.trays.get(&0).ok_or_else(|| "Tray not found".to_string())?;
        tray.get_dice()
            .iter()
            .map(|id| allocator.dice[id].to_summary())
            .collect::<Vec<DieSummary>>()
    };

    let sorted = {
        let tray = allocator.trays.get_mut(&0).ok_or_else(|| "Tray not found".to_string())?;
        tray.sort(summaries, Ordering::Less)
    };

    let sorted_ids = sorted.iter().map(|die| die.get_id()).collect::<Vec<usize>>();
    assert_eq!(sorted_ids, vec![1, 3, 2, 0]);

    Ok(())
}

#[test]
fn test_allocator_sort_tray() -> Result<(), String> {
    let mut allocator = Allocator::new();
    allocator.new_tray(Some("Allocator Sort Tray".to_string()));

    allocator.new_die(20, Some(101), Some("d20".to_string()))?;
    allocator.new_die(4, Some(202), Some("d4".to_string()))?;
    allocator.new_die(12, Some(303), Some("d12".to_string()))?;
    allocator.new_die(6, Some(404), Some("d6".to_string()))?;

    allocator.move_die(0, Some(0))?;
    allocator.move_die(1, Some(0))?;
    allocator.move_die(2, Some(0))?;
    allocator.move_die(3, Some(0))?;

    let sorted_asc = allocator.sort_tray(0, Ordering::Less)?;
    let asc_ids = sorted_asc.iter().map(|die| die.get_id()).collect::<Vec<usize>>();
    assert_eq!(asc_ids, vec![1, 3, 2, 0]);

    let sorted_desc = allocator.sort_tray(0, Ordering::Greater)?;
    let desc_ids = sorted_desc.iter().map(|die| die.get_id()).collect::<Vec<usize>>();
    assert_eq!(desc_ids, vec![0, 2, 3, 1]);

    let tray_ids = allocator
        .trays
        .get(&0)
        .ok_or_else(|| "Tray not found".to_string())?
        .get_dice()
        .clone();
    assert_eq!(tray_ids, desc_ids);

    Ok(())
}


