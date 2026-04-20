use crate::die::{DiceDataList, Die, DieData, RollLog, build_die};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::Display;

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

pub struct DieAllocator{
    tray_id_gen: TrayIDGenerator,
    die_id_gen: DieIdGenerator,
    dice: HashMap<usize, Die>,
    trays: HashMap<usize, DieTray>
}

impl DieAllocator{
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
        // Find the die by id
        let die = self.dice.get_mut(&die_id)
            .ok_or_else(|| format!("No die found with ID: {}", die_id))?;

        let die_id = die.get_id();

        if let Some(tray) = self.trays.get_mut(&die.get_id()){
            tray.remove_die(die_id)?;
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

    pub fn roll_die(&mut self, die_id: usize) -> Result<RollLog, String> {
        if let Some(selected_die) = self.dice.get_mut(&die_id){
            return Ok(selected_die.roll());
        }
        else {
            Err(format!("No die found with given die ID: {} Cannot roll.", die_id))
        }
    }

    pub fn print_dice(&self){
        println!("---ALL DICE IN ALLOCATOR---");
        for die in self.dice.values(){
            println!{"{}", die};
        }
    }

    pub fn print_tray(&self, tray_id: usize) -> Result<(), String> {
        let tray = self.trays.get(&tray_id)
            .ok_or_else(|| format!("No tray found with ID: {}", tray_id))?;

        println!("Found tray with {}", tray_id);
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
    let mut allocator = DieAllocator::new();
    allocator.new_tray(Some("Best Tray".to_string()));
    allocator.new_tray(Some("Worst Tray".to_string()));
    allocator.new_tray(None);

    for t in allocator.trays.values(){
        println!("{}", t);
    }
}

fn build_allocator_from_file() -> Result<DieAllocator, String> {
    use std::fs;
    use std::path::PathBuf;
    use dotenv;

    dotenv::from_filename("rust_dice/src/.env").unwrap();

    let rel = PathBuf::from(std::env::var("DICE_DATA_PATH").unwrap());
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);
    let file_path = data_dir.join("die_test");
    let die_file = fs::read_to_string(&file_path).unwrap();

    let decoded_list: DiceDataList = serde_json::from_str(&die_file).unwrap();
    
    let mut allocator = DieAllocator::new();
    allocator.new_tray(Some("THE TRAY".to_string()));
    allocator.new_dice_from_list(decoded_list).unwrap();
    Ok(allocator)
}

#[test]
fn test_dice_from_list() -> Result<(), String>{
    let allocator = build_allocator_from_file().unwrap();
    allocator.print_dice();
    Ok(())
}


#[test]
fn test_dice_to_tray() -> Result<(), String>{
    let mut allocator = build_allocator_from_file().unwrap();
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


