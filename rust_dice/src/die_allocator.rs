use crate::die::{Die, RollLog, build_die};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

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
    pub fn new() -> Self{
        Self { 
            tray_id_gen: TrayIDGenerator::new(),
            die_id_gen: DieIdGenerator::new(),
            dice: HashMap::new(),
            trays: HashMap::new() 
        }
    }

    pub fn new_tray(&mut self){
        let new_tray_id = self.tray_id_gen.get_next_tray_id();
        self.trays.insert(new_tray_id, DieTray::new(new_tray_id));        
    }

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

    pub fn add_to_tray(&mut self, die_id: usize, tray_id: usize) -> Result<(), String>{
        // Find the die by id
        let die = self.dice.get(&die_id)
            .ok_or_else(|| format!("No die found with ID: {}", die_id))?;

        // Find the tray by id
        let tray = self.trays.get_mut(&tray_id)
            .ok_or_else(|| format!("No tray found with ID: {}", tray_id))?;

        // Add die reference to tray
        tray.add_die(die);
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
}

pub struct DieTray{
    id: usize,
    dice: Vec<usize>
}

impl DieTray{
    pub fn new(tray_id: usize) -> Self{
        DieTray { 
            id: tray_id,
            dice: Vec::new()
        }
    }

    fn add_die(&mut self, die : &Die){
        self.dice.push(die.get_id());
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



