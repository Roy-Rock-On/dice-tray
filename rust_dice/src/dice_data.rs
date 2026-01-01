
use serde::{Serialize, Deserialize};
use crate::dice::{Die, Die32, DieResult, DieResultType};
use crate::tray::Tray;

///Trait ensures that DieData can be created by any type that implements the Die trait.
pub trait DieData: Serialize + for<'a> Deserialize<'a> {
    fn from_die(die: &dyn Die) -> TypedDieData;
}

///Enum is used to flag the die type to be created from die data and ensured the right data is used to build the right die.
#[derive(Serialize, Deserialize, Clone)]
pub enum TypedDieData {
    Die32(DieData32),
}

impl TypedDieData{
    ///TypedDie data can be converted into a boxed ref to a die. With the given ID.
    pub fn to_die(self, id: usize) -> Box<dyn Die>{
        match self{
            TypedDieData::Die32(data) => {
                Box::new(Die32::from_data(id, &data))
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DieData32{
    rng_seed: u64,
    label: String,
    faces: u32,
    current_face: u32,
    current_result: DieResult,
    current_result_type: DieResultType
}

impl DieData for DieData32 {
    fn from_die(die: &dyn Die) -> TypedDieData {
        TypedDieData::Die32(
            DieData32{
                rng_seed: die.get_rng_seed(),
                label: die.get_label().to_string(),
                faces: die.get_face_count(),
                current_face: die.get_current_face() as u32,
                current_result: die.get_result().clone(),
                current_result_type: die.get_result_type().clone()
            })
    }    
}

impl DieData32{
    ///Gets the rng seed form the data- making the dice consistant across sessions (sort of).
    pub fn get_seed(&self) -> u64 {
        self.rng_seed
    }

    ///Gets the dice data label as a string slice.
    pub fn get_label(&self) -> &str{
        &self.label
    }

    ///Gets the faces saved to the die data as a u32.
    pub fn get_faces(&self) -> u32{
        self.faces
    }

    ///Gets the current face from the die data as a u32. 
    pub fn get_current_face(&self) -> u32{
        self.current_face
    }

    ///Gets the current result form the die data as a DieResult
    pub fn get_current_result(&self) -> &DieResult{
        &self.current_result
    }

    ///Gets the current result type as a DieResultType
    pub fn get_current_result_type(&self) -> &DieResultType{
        &self.current_result_type
    }
}

/// Tray data is used to save/load dice trays using the serde crate. 
/// Enforces that TrayData Types must implement From<&dyn Tray>
pub trait TrayData<'a>: From<&'a dyn Tray> + Serialize + Deserialize<'a> {}


