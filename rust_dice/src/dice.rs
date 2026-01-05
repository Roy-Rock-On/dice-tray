use rand::rngs::SmallRng;
use rand::{Rng, RngCore, SeedableRng};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::write;
use std::mem::discriminant;

use crate::dice_data::DieData32;

/// Used to type dice for serilization/deserilization.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DieType {
    Die32,
}

/// The die trait alows for extending this library with custom dice types.
pub trait Die{
    //&self
    ///Returns a u64 that can be used to generate new RNG the next time the die is instantiated.
    fn get_rng_seed(&self) -> u64;

    ///Returns the die type, handled by the constructor of the die struct. Used to serialize and deserialize dice.
    fn get_die_type(&self) -> &DieType;

    ///Gets the die ID- which should be a unique key that allows for reactivity in leptos, key/value stores, etc.
    fn get_id(&self) -> usize;

    ///Gets a label used to identify the die. Unlike ID many dice can share the same label. Lables can be used to identify dice in a group.
    fn get_label(&self) -> &str;

    ///Gets the number of faces on the die.
    fn get_face_count(&self) -> u32;

    ///Gets the current face of the die.
    fn get_current_face(&self) -> i32;

    ///Gets the value of the face. Normally this is the same as the current face, but custom dice can define their own values to assing to the die faces.
    fn get_face_value(&self) -> i32;

    ///Returns the current result of the die as a DieResult. This is dependent on both the die and result type.
    fn get_result(&self) -> &DieResult;

    ///Used to get a reffrence to the current result type of the die.
    fn get_result_type(&self) -> &DieResultType;

    ///Returns true if the die's current face is the face with the highest value.m
    fn is_max(&self) -> bool;

    ///Returns true if thr die's current face is the face with the lowest value.
    fn is_min(&self) -> bool;

    ///Gets a summary of the dice as a string. Used to quickly print a summary of a dice tray.
    fn get_summary(&self) -> String;

    //&mut self
    ///Rolls the die.
    fn roll(&mut self, result_type: Option<DieResultType>);

    ///Increments the face on the die by one, if face is maxed wrap the die around to one.
    fn increment(&mut self);

    ///Decrements the face of the die, if the die face is 1 wraps up to the max face.
    fn decrement(&mut self);

    ///Sets the face of the die to the new_face value. Clamps the value within the range of the die's faces.
    fn set_face(&mut self, new_face: i32);
}

/// Represents a physical dice. Includes a string identifier, it's own SmallRng seed, ability to roll and compare rolls.
#[derive(Debug, Clone)]
pub struct Die32 {
    die_type: DieType,
    id: usize,
    rng: SmallRng,
    label: String,
    faces: u32,
    current_face: u32,
    current_result: DieResult,
    result_type: DieResultType,
}

impl Die for Die32 {
    fn get_die_type(&self) -> &DieType {
        &self.die_type
    }

    fn get_rng_seed(&self) -> u64 {
        self.rng.clone().next_u64()
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_label(&self) -> &str {
        &self.label
    }

    fn get_face_count(&self) -> u32 {
        self.faces
    }

    fn get_current_face(&self) -> i32 {
        self.current_face as i32
    }

    fn get_face_value(&self) -> i32 {
        self.get_current_face()
    }

    fn get_result(&self) -> &DieResult {
        &self.current_result
    }

    fn get_result_type(&self) -> &DieResultType {
        &self.result_type
    }

    fn get_summary(&self) -> String {
        format!("&{} = {} ", self.label, self.get_result().to_string())
    }

    fn roll(&mut self, result_type: Option<DieResultType>) {
        if let Some(result_type) = result_type {
            self.set_result_type(result_type);
        }
        self.current_face = self.rng.random_range(1..=self.faces);
        self.update_result();
    }

    fn is_max(&self) -> bool {
        self.current_face == self.faces
    }

    fn is_min(&self) -> bool {
        self.current_face == 1
    }

    fn increment(&mut self) {
        self.current_face += 1;
        if self.current_face > self.faces {
            self.current_face = 1;
        }
    }

    fn decrement(&mut self) {
        self.current_face -= 1;
        if self.current_face < 1 {
            self.current_face = self.faces
        }
    }

    fn set_face(&mut self, face: i32) {
        let mut new_face = face as u32;
        if new_face < 1 {
            new_face = 1;
        } else if new_face > self.faces {
            new_face = self.faces
        }
        self.current_face = new_face;
    }
}

impl Die32 {
    /// Creates a new die, with an optional string label.
    /// If no identity is provided will default to the dice notation number (e.g. 'd6', 'd100').
    /// If a identity is provided, the dice will check the settings for a result table with that name.
    /// This lets you setup dice that automatically lookup results in a table allowing for custom dice faces.
    /// The new dice is rolled on creation to give it a random self_current face.
    pub fn new(
        id: usize,
        label: Option<String>,
        faces: u32,
        result_type: Option<DieResultType>,
    ) -> Self {
        let new_result_type = match result_type {
            Some(r) => r,
            None => DieResultType::Face,
        };

        let mut new_die = Die32 {
            die_type: DieType::Die32,
            id,
            rng: SmallRng::from_rng(&mut rand::rng()),
            label: label.unwrap_or_else(|| "d".to_string() + &faces.to_string()),
            faces,
            current_face: 1,
            current_result: DieResult::Number(1),
            result_type: new_result_type,
        };

        new_die.roll(None);
        new_die
    }

    ///Creates a new Die32 from Die32 data - allows for saving dice between sessions as certian fields (i.e. RNG) can't be serialized with serde.
    ///ID must be provided by the dice allocator and the die will get a new RNG seed.
    pub fn from_data(id: usize, data: &DieData32) -> Self {
        Die32 {
            die_type: DieType::Die32,
            id,
            rng: SmallRng::seed_from_u64(data.get_seed()),
            label: data.get_label().to_string(),
            faces: data.get_faces(),
            current_face: data.get_current_face(),
            current_result: data.get_current_result().clone(),
            result_type: *data.get_current_result_type(),
        }
    }

    pub fn set_current_face(&mut self, face: u32) {
        self.current_face = self.clamp_to_bounds(face);
    }

    /// When setting the face of a Die this will clamp the value to a the valid range, if required.
    fn clamp_to_bounds(&self, value: u32) -> u32 {
        if value < 1 {
            1
        } else if value > self.faces {
            self.faces
        } else {
            value
        }
    }

    /// Sets the result type of the die to the provided DieResultType and updates the current result accordingly.
    fn set_result_type(&mut self, new_result_type: DieResultType) {
        //gaurd against changeing the result type if we don't have to.
        if self.result_type == new_result_type {
            return;
        }

        self.current_result = match new_result_type {
            DieResultType::Best => DieResult::Number(1),
            DieResultType::Worst => DieResult::Number(self.faces),
            DieResultType::Sum => DieResult::Number(0),
            DieResultType::Face => DieResult::Number(0),
        };

        self.result_type = new_result_type;
        self.update_result();
    }

    fn update_result(&mut self) {
        match self.result_type {
            DieResultType::Face => {
                self.current_result = DieResult::Number(self.current_face);
            }
            DieResultType::Best => {
                let last_result = self.current_result.is_num_or(1);
                if self.current_face > last_result {
                    self.current_result = DieResult::Number(self.current_face);
                }
            }
            DieResultType::Worst => {
                let last_result = self.current_result.is_num_or(self.faces);
                if self.current_face < last_result {
                    self.current_result = DieResult::Number(self.current_face);
                }
            }
            DieResultType::Sum => {
                self.current_result =
                    DieResult::Number(self.current_result.is_num_or(0) + self.current_face);
            }
        }
    }
}

/// Ordering for Die ignores RNG state and uses (current_face, faces, label)
impl PartialEq for Die32 {
    fn eq(&self, other: &Self) -> bool {
        (self.current_face, self.faces, &self.label)
            == (other.current_face, other.faces, &other.label)
    }
}

impl Eq for Die32 {}

impl PartialOrd for Die32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Die32 {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.current_face, self.faces, &self.label).cmp(&(
            other.current_face,
            other.faces,
            &other.label,
        ))
    }
}

/// Used to request specific result types from a Die roll.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DieResultType {
    Face,
    Best,
    Worst,
    Sum,
}

impl ToString for DieResultType {
    fn to_string(&self) -> String {
        match self {
            DieResultType::Face => "Face".to_string(),
            DieResultType::Best => "Best".to_string(),
            DieResultType::Worst => "Worst".to_string(),
            DieResultType::Sum => "Sum".to_string(),
        }
    }
}

impl PartialEq for DieResultType {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

/// Used to return specific result types from a Die roll and wraps the returned value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DieResult {
    Number(u32),
    String(String),
    None,
}

impl DieResult {
    /// Checks if the DieResult is a number type, otherwise defaults the die result to the provided default.
    pub fn is_num_or(&self, default_num: u32) -> u32 {
        match self {
            DieResult::Number(x) => *x,
            _ => default_num,
        }
    }

    pub fn to_string(&self) -> String{
        match self{
            DieResult::Number(num) => num.to_string(),
            DieResult::String(string) => string.to_string(),
            DieResult::None => "".to_string()
        }
    }
}
