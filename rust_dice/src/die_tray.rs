use crate::id_generator::IdGenerator;
use crate::die_reader::DieReader;
use crate::die::Die;

use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt::Display;

use serde::{Serialize, Deserialize};

pub struct DieTray{
    id: usize,
    label: String,
    readers: Vec<DieReader>,
    reader_id_gen: IdGenerator
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
            readers: Vec::new(),
            reader_id_gen: IdGenerator::new()
        }
    }
    
    pub fn roll_all(&mut self, dice: &mut HashMap<usize, Die>) -> Result<TraySummary, String>{
        for reader in self.readers.iter_mut(){
            let die_id = reader.get_die_id();
            if let Some(die) = dice.get_mut(&die_id){
                die.roll()
            };
        }

        Ok(self.build_summary())
    }
    
    pub fn roll_at(&mut self, reader_ids: &[usize], dice: &mut HashMap<usize, Die>) -> Result<TraySummary, String>{
        self.readers.iter_mut()
            .filter(|read| reader_ids.contains(&read.get_reader_id()))
            .for_each(|r|{
                let die_id: usize = r.get_die_id();
                if let Some(die) = dice.get_mut(&die_id){
                    die.roll();
                };
            });

        Ok(self.build_summary())
    }

    pub fn sort(&mut self, order: Ordering){
        match order {
            Ordering::Equal => (),
            Ordering::Greater => self.readers.sort_by(|a, b| b.cmp(a)),
            Ordering::Less => self.readers.sort_by(|a, b| a.cmp(b))
        }
    }

    pub fn build_summary(&self) -> TraySummary {
        TraySummary::new(self)
    }

    pub fn add_reader(&mut self, die : &Die){
        let next_id = self.reader_id_gen.allocate();
        let new_reader = DieReader::new(die, next_id);
        self.readers.push(new_reader);
    }

    pub fn remove_readers_by_die_id(&mut self, die_id: usize){
        let targeted_reader_ids : Vec<usize> = self.readers.iter()
            .filter_map(|r|{
                if r.get_die_id() == die_id{
                    Some(r.get_reader_id())
                }
                else{
                    None
                }
            }).collect();

        for id in targeted_reader_ids{
            self.reader_id_gen.free(id);
        }

        self.readers.retain(|dr| dr.get_reader_id() != die_id);
    }

    pub fn remove_readers_by_reader_id(&mut self, reader_ids: &mut [usize]) {
        for id in reader_ids.iter(){
            self.reader_id_gen.free(*id);
        }

        self.readers.retain(|r| !reader_ids.contains(&r.get_reader_id()))           
    }

    pub fn clear_readers(&mut self){
        self.readers.clear();
    }
}

impl Display for DieTray{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tray ID = {}, Tray Label = {}, Count of dice in tray = {}",
            self.id,
            self.label,
            self.readers.len()
        )
    }
}


#[derive(Serialize, Deserialize)]
///A tray summary is used to sync the tray with a frontend UI.
pub struct TraySummary{
    tray_id: usize,
    tray_label: String,
    tray_dice: Vec<DieReader>
}

impl TraySummary{
    pub fn new(tray: &DieTray) -> Self {
        TraySummary { 
            tray_id: tray.id, 
            tray_label: tray.label.to_string(),
            tray_dice: tray.readers.clone()
        }
    }

    pub fn get_dice(&self) -> &Vec<DieReader>{
        &self.tray_dice
    }

    pub fn get_label(&self) -> &str{
        &self.tray_label
    }

    pub fn get_id(&self) -> usize{
        self.tray_id
    }

    pub fn print(&self){
        println!("---{}---", self.tray_label);
        println!("Tray ID = {}", self.tray_id);
        for reader in self.tray_dice.iter().enumerate(){
            println!("@{} - Faces {} - Result {}", reader.0, reader.1.get_face_count(), reader.1.get_current_face());
        }
    }
}