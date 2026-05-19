use crate::die_targets::DiceTargets;
use crate::id_generator::IdGenerator;
use crate::die_reader::{self, DieReader};
use crate::die::Die;

use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use std::fmt::Display;

use serde::{Serialize, Deserialize};

pub struct DieTray{
    label: String,
    readers: Vec<DieReader>,
    reader_id_gen: IdGenerator
}

impl DieTray{
    pub fn new(tray_name: String) -> Self{       
        DieTray { 
            label: tray_name,
            readers: Vec::new(),
            reader_id_gen: IdGenerator::new()
        }
    }
    
    pub fn roll_all(&mut self, dice: &mut HashMap<usize, Die>) -> anyhow::Result<TraySummary>{
        for reader in self.readers.iter_mut(){
            let die_id = reader.get_die_id();
            if let Some(die) = dice.get_mut(&die_id){
                die.roll()
            };
        }

        Ok(self.build_summary())
    }
    
    pub fn roll_at(&mut self, reader_ids: &[usize], dice: &mut HashMap<usize, Die>) -> anyhow::Result<TraySummary>{
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

    pub fn add_reader(&mut self, die : &Die) -> usize{
        let next_id = self.reader_id_gen.allocate();
        let new_reader = DieReader::new(die, next_id);
        self.readers.push(new_reader);
        next_id
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

    pub fn remove_readers(&mut self, reader_ids: &Vec<usize>) -> Option<Vec<usize>>{      
        for id in reader_ids{
            self.reader_id_gen.free(*id);
        }

        let die_ids : Vec<usize> = self.readers.iter()
            .filter_map(|dr| Some(dr.get_die_id()))
            .collect();

        self.readers.retain(|r| !reader_ids.contains(&r.get_reader_id()));

        if die_ids.len() > 0{
            Some(die_ids)
        }
        else{
            None
        }
    }

    pub fn clear_readers(&mut self){
        self.reader_id_gen.free_all();
        self.readers.clear();
    }

    pub fn get_reader_ids_by_targets(&self, targets: &DiceTargets) -> Option<Vec<usize>>{
        let mut found_ids = HashSet::new();

        match targets {
            DiceTargets::All => {
                for reader in &self.readers{
                    found_ids.insert(reader.get_reader_id());
                }
            },
            DiceTargets::Index(indecies) => {
                for reader in &self.readers{
                    if indecies.contains(&reader.get_reader_id()){
                        found_ids.insert(reader.get_reader_id());
                    }
                }
            },
            DiceTargets::Label(labels) => {
                for reader in &self.readers{
                    if labels.contains(&reader.get_label().to_string()){
                        found_ids.insert(reader.get_reader_id());
                    }
                }
            },
            DiceTargets::None => ()
        }

        let found_ids: Vec<usize> = found_ids.into_iter().collect();

        if found_ids.len() > 0{
            Some(found_ids)
        }
        else{
            None
        }
    }

    pub fn get_label(&self) -> &str{
        &self.label
    }
}

impl Display for DieTray{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tray Label = {}, Count of dice in tray = {}",
            self.label,
            self.readers.len()
        )
    }
}


#[derive(Serialize, Deserialize)]
///A tray summary is used to sync the tray with a frontend UI.
pub struct TraySummary{
    tray_label: String,
    tray_dice: Vec<DieReader>
}

impl TraySummary{
    pub fn new(tray: &DieTray) -> Self {
        TraySummary { 
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

    pub fn print(&self){
        println!("---{}---", self.tray_label);
        for reader in self.tray_dice.iter().enumerate(){
            println!("@{} - Faces {} - Result {}", reader.0, reader.1.get_face_count(), reader.1.get_current_face());
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MoveSummary{
    pub from_tray: TraySummary,
    pub to_tray: Option<TraySummary>
}

impl MoveSummary{
    pub fn new(from: TraySummary, to: Option<TraySummary>) -> Self {
        MoveSummary{
            from_tray: from,
            to_tray: to
        }
    }
}