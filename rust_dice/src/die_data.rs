use serde::{Serialize, Deserialize};

///Enum used to control the sorting of dice states.
pub enum DieSort{
    FaceCount,
    CurrentFace
}

/// Die state represents the current state of the die. 
/// This is used to allow many DieReaders to use the same underlying Die.
#[derive(Serialize, Deserialize, Debug)]
pub struct DieState{
    pub id: usize,
    pub label: String,
    pub faces: u32,
    pub current_face: u32
}

impl DieState{
    pub fn print(&self){
        println!("Die State:");
        println!("id: {} | label: {} | faces: {} | current_face: {}", self.id, self.label, self.faces, self.current_face);
    }
}

///Wrapper struct for a group of Dice States.
#[derive(Serialize, Deserialize, Debug)]
pub struct DiceState{
    dice: Vec<DieState>
}

impl DiceState{
    pub fn new(die_states: Vec<DieState>) -> Self{
        DiceState{
            dice: die_states
        }
    }

    pub fn sort(&mut self, sort_method: DieSort){
        match sort_method {
            DieSort::FaceCount => {
                self.dice.sort_by(|a, b|{
                    a.faces.cmp(&b.faces)
                })
            },
            DieSort::CurrentFace => {
                self.dice.sort_by(|a, b|{
                    a.current_face.cmp(&b.current_face)
                })
            }
        }

    }
}


/// DieData is used to serialize and deserialize a die's underlying RNG.
/// This is used to save/load dice between sessions.  
#[derive(Serialize, Deserialize)]
pub struct DieData{
    pub label: String,
    pub current_face: u32,
    pub face_weights: Vec<u32>,
    pub total_weight: u32,
    pub last_rng_seed: u64,
}

///DiceDataList is a wrapper for DieData. Uses to bulk save/load dice.
#[derive(Serialize, Deserialize)]
pub struct DiceDataList{
    pub dice_data_vec: Vec<DieData>
}

impl DiceDataList{
    ///Creates a new dice data list for serialization into JSON.
    ///Lets dice be saved between sessions. 
    pub fn new() -> Self{
        DiceDataList { 
            dice_data_vec: Vec::new() 
        }
    }

    ///Allows for adding data to a dice data list. 
    ///Used to iterate through a series of dice for serialization.
    pub fn add_data(&mut self, die_data: DieData){
        self.dice_data_vec.push(die_data);
    }
}

