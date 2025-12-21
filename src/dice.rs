use crate::settings::DICE_TRAY_SETTINGS;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::cmp::Ordering;
use std::mem::discriminant;

/// Used to request specific result types from a Die roll.
#[derive(Debug, Clone, Copy)]
pub enum DieResultType {
    Face,
    Best,
    Worst,
    Sum,
    Table,
}

impl PartialEq for DieResultType {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

/// Used to return specific result types from a Die roll and wraps the returned value.
#[derive(Debug, Clone)]
pub enum DieResult {
    Number(u32),
    String(String),
    None,
}

/// Represents a physical dice. Includes a string identifier, it's own SmallRng seed, ability to roll and compare rolls.
#[derive(Debug, Clone)]
pub struct Die {
    rng: SmallRng,
    identity: String,
    faces: u32,
    current_face: u32,
    current_result_value: Option<u32>,
    result_type: DieResultType,
}

impl Die {
    /// Creates a new die, with an optional string identifier.
    /// If no identity is provided will default to the dice notation number (e.g. 'd6', 'd100').
    /// If a identity is provided, the dice will check the settings for a result table with that name.
    /// This lets you setup dice that automatically lookup results in a table allowing for custom dice faces.
    /// The new dice is rolled on creation to give it a random self_current face.
    pub fn new(identity: Option<String>, faces: u32) -> Self {
        let result_type = {
            if let Some(id) = identity.as_ref() {
                let settings = DICE_TRAY_SETTINGS.lock().unwrap();
                if settings.has_table(&id) {
                    println!("Found a table for {}", id);
                    DieResultType::Table
                } else {
                    DieResultType::Face
                }
            } else {
                DieResultType::Face
            }
        };

        let mut new_die = Die {
            rng: SmallRng::from_rng(&mut rand::rng()),
            identity: identity.unwrap_or_else(|| "d".to_string() + &faces.to_string()),
            faces,
            current_face: 1,
            current_result_value: Some(1),
            result_type,
        };

        new_die.roll(result_type);
        new_die
    }

    /// Returns the ID of the die as a string slice.
    pub fn get_id(&self) -> &str {
        &self.identity
    }

    /// Returns the result of the die roll based on the result type of the die.
    pub fn get_result(&self) -> DieResult {
        match &self.result_type {
            DieResultType::Face
            | DieResultType::Best
            | DieResultType::Worst
            | DieResultType::Sum => DieResult::Number(self.current_result_value.unwrap_or(0)),
            DieResultType::Table => {
                let settings = DICE_TRAY_SETTINGS.lock().unwrap();
                let result = settings.dice_table_lookup(self.identity.as_str(), self.current_face);
                match result {
                    Ok(result_str) => DieResult::String(result_str.to_string()),
                    Err(_) => DieResult::String("No result found.".to_string()),
                }
            }
        }
    }

    /// Rolls the dice to give it a random number between 1 and self.faces (inclusive).
    pub fn roll(&mut self, result_type: DieResultType) {
        self.set_result_type(result_type);
        self.current_face = self.rng.random_range(1..=self.faces);
        self.update_result();
    }

    /// Rertuns the current face of the dice as a u32.
    pub fn get_current_face(&self) -> u32 {
        self.current_face
    }

    pub fn set_current_face(&mut self, face: u32) {
        self.current_face = self.clamp_to_bounds(face);
    }

    /// Returns the identity of the dice as a string slice.
    pub fn get_identity(&self) -> &str {
        &self.identity
    }

    /// Sets the identity of the dice to the provided string.
    pub fn set_identity(&mut self, identity: String) {
        self.identity = identity;
    }

    /// Used to get the current result value of the die.
    pub fn get_result_value(&self) -> Option<u32> {
        self.current_result_value
    }

    /// Returns the result type of the die.
    pub fn get_result_type(&self) -> &DieResultType {
        &self.result_type
    }

    /// Returns true if the current face is the minimum value (1).
    pub fn is_min(&self) -> bool {
        self.current_face == 1
    }

    /// Returns true if the current face is the maximum value (self.faces).
    pub fn is_max(&self) -> bool {
        self.current_face == self.faces
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
        //Don't do anything if we don't have too.
        if self.result_type == new_result_type || self.result_type == DieResultType::Table {
            println!(
                "Die {} is already set, or the die is tied to a lookup table.",
                self.identity
            );
            return;
        }

        self.current_result_value = match new_result_type {
            DieResultType::Table => None,
            DieResultType::Best => Some(1),
            DieResultType::Worst => Some(self.faces),
            DieResultType::Sum => Some(0),
            DieResultType::Face => Some(0),
        };

        self.result_type = new_result_type;
        self.update_result();
    }

    fn update_result(&mut self) {
        match self.result_type {
            DieResultType::Face => {
                self.current_result_value = Some(self.current_face);
            }
            DieResultType::Best => {
                let last_result = self.current_result_value.unwrap_or(1);
                if self.current_face > last_result {
                    self.current_result_value = Some(self.current_face);
                }
            }
            DieResultType::Worst => {
                let last_result = self.current_result_value.unwrap_or(self.faces);
                if self.current_face < last_result {
                    self.current_result_value = Some(self.current_face);
                }
            }
            DieResultType::Sum => {
                self.current_result_value =
                    Some(self.current_result_value.unwrap_or(0) + self.current_face);
            }
            DieResultType::Table => {
                self.current_result_value = None; // Table results are handled separately, a result value of 0 means table results won't impact
            }
        }
    }
}

/// Creates a vector of dice with a count equal to conunt and a number of faces equal to faces.
pub fn new_dice_set(count: u32, faces: u32) -> Vec<Die> {
    let mut new_die_vec = Vec::new();
    for _i in 0..count {
        let new_die = Die::new(None, faces);
        new_die_vec.push(new_die);
    }
    new_die_vec
}

/// Creates a vector of dice from a given vector of u32. Each die has a face count equal each u32 in the count.
pub fn new_dice_from_vec(dice: Vec<u32>) -> Vec<Die> {
    dice.iter().map(|f| Die::new(None, *f)).collect()
}

pub fn new_dice_from_vec_with_id(dice: Vec<(Option<String>, u32)>) -> Vec<Die> {
    dice.iter().map(|i| Die::new(i.0.clone(), i.1)).collect()
}

/// Ordering for Die ignores RNG state and uses (current_face, faces, identity)
impl PartialEq for Die {
    fn eq(&self, other: &Self) -> bool {
        (self.current_face, self.faces, &self.identity)
            == (other.current_face, other.faces, &other.identity)
    }
}

impl Eq for Die {}

impl PartialOrd for Die {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Die {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.current_face, self.faces, &self.identity).cmp(&(
            other.current_face,
            other.faces,
            &other.identity,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_die_creation_and_roll() {
        let faces = 20;
        let mut die = Die::new(Some("TestDie".to_string()), faces);
        assert_eq!(die.get_identity(), "TestDie");
        assert!(die.get_current_face() >= 1 && die.get_current_face() <= faces);
        let first_roll = die.get_current_face();
        die.roll(DieResultType::Face);
        println!(
            "First roll: {}, Second roll: {}",
            first_roll,
            die.get_current_face()
        );
    }

    #[test]
    fn dice_ordering() {
        let die1 = Die::new(None, 6);
        let die2 = Die::new(None, 6);
        let die3 = Die::new(None, 6);
        let die4 = Die::new(None, 6);
        let die5 = Die::new(None, 6);
        let die6 = Die::new(None, 6);

        let mut dice = Vec::new();
        dice.push(die1);
        dice.push(die2);
        dice.push(die3);
        dice.push(die4);
        dice.push(die5);
        dice.push(die6);

        let dice_results: Vec<u32> = dice.iter().map(|d| d.get_current_face()).collect();
        println!("Dice in inserted order: {:?}", dice_results);
        dice.sort();
        let dice_results = dice
            .iter()
            .map(|d| d.get_current_face())
            .collect::<Vec<u32>>();
        println!("Dice in sorted order: {:?}", dice_results);
    }
}
