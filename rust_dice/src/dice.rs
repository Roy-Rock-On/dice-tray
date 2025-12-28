use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::cmp::Ordering;
use std::mem::discriminant;



/// The die trait alows for extending this library with custom dice types.
pub trait Die {
    ///Gets the die ID- which should be a unique key that allows for reactivity in leptos.
    fn get_id(&self) -> usize;

    ///Gets a label used to identify the die. Unlike ID many dice can share the same label. Lables can be used to identify dice in a group.
    fn get_label(&self) -> &str;

    ///Gets the current face value of the die.
    fn get_current_face_value(&self) -> u32;

    ///Returns the current result of the die as a DieResult. This is dependatn on both the die and result type.
    fn get_result(&self) -> DieResult;

    ///Used to get a reffrence to the current result type of the die.
    fn get_result_type(&self) -> &DieResultType;

    ///Rolls the die.
    fn roll(&mut self, result_type: DieResultType);

    ///Returns true if the die's current face is the face with the highest value.
    fn is_max(&self) -> bool;

    ///Returns true if thr die's current face is the face with the lowest value.
    fn is_min(&self) -> bool;
}

/// Represents a physical dice. Includes a string identifier, it's own SmallRng seed, ability to roll and compare rolls.
#[derive(Debug, Clone)]
pub struct Die32 {
    id: usize,
    rng: SmallRng,
    label: String,
    faces: u32,
    current_face: u32,
    current_result_value: Option<u32>,
    result_type: DieResultType,
}

impl Die for Die32{
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_label(&self) -> &str {
        &self.label
    }

    fn get_current_face_value(&self) -> u32 {
        self.current_face
    }

    fn get_result(&self) -> DieResult {
        DieResult::Number(self.current_result_value.unwrap_or(0))
    }

    fn get_result_type(&self) -> &DieResultType {
        &self.result_type
    }

    fn roll(&mut self, result_type :DieResultType) {
        self.set_result_type(result_type);
        self.current_face = self.rng.random_range(1..=self.faces);
        self.update_result();
    }

    fn is_max(&self) -> bool {
        self.current_face == self.faces
    }

    fn is_min(&self) -> bool {
        self.current_face == 1
    }
}

impl Die32 {
    /// Creates a new die, with an optional string label.
    /// If no identity is provided will default to the dice notation number (e.g. 'd6', 'd100').
    /// If a identity is provided, the dice will check the settings for a result table with that name.
    /// This lets you setup dice that automatically lookup results in a table allowing for custom dice faces.
    /// The new dice is rolled on creation to give it a random self_current face.
    pub fn new(id: usize, label: Option<String>, faces: u32) -> Self {
        let mut new_die = Die32 {
            id,
            rng: SmallRng::from_rng(&mut rand::rng()),
            label: label.unwrap_or_else(|| "d".to_string() + &faces.to_string()),
            faces,
            current_face: 1,
            current_result_value: Some(1),
            result_type : DieResultType::Face,
        };

        new_die.roll(DieResultType::Face);
        new_die
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
        self.current_result_value = match new_result_type {
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
#[derive(Debug, Clone, Copy)]
pub enum DieResultType {
    Face,
    Best,
    Worst,
    Sum,
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

impl DieResult {
/// Checks if the DieResult is a number type, otherwise defaults the die result to the provided default.
    pub fn is_num_or(&self, default_num : u32) -> u32{
        match self{
            DieResult::Number(x) => *x,
            _ => default_num
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dice_builders::*;
    use crate::dice_profile::{DieProfile, DieType};

    #[test]
    fn test_die_creation_and_roll() {
        let faces = 20;
        let mut die = new_die(0, &DieProfile::new(None, DieType::Numerical(faces)));
        assert_eq!(die.get_label(), "TestDie");
        assert!(die.get_current_face_value() >= 1 && die.get_current_face_value() <= faces);
        let first_roll = die.get_current_face_value();
        die.roll(DieResultType::Face);
        println!(
            "First roll: {}, Second roll: {}",
            first_roll,
            die.get_current_face_value()
        );
    }

    #[test]
    fn dice_ordering() {
        let die1 = new_die(1, &DieProfile::new(None, DieType::Numerical(6)));
        let die2 = new_die(2, &DieProfile::new(None, DieType::Numerical(6)));
        let die3 = new_die(3, &DieProfile::new(None, DieType::Numerical(6)));
        let die4 = new_die(4, &DieProfile::new(None, DieType::Numerical(6)));
        let die5 = new_die(5, &DieProfile::new(None, DieType::Numerical(6)));
        let die6 = new_die(6, &DieProfile::new(None, DieType::Numerical(6)));

        let mut dice: Vec<Box<dyn Die>> = Vec::new();
        dice.push(Box::new(die1));
        dice.push(Box::new(die2));
        dice.push(Box::new(die3));
        dice.push(Box::new(die4));
        dice.push(Box::new(die5));
        dice.push(Box::new(die6));

        let dice_results: Vec<u32> = dice.iter().map(|d| d.get_current_face_value()).collect();
        println!("Dice in inserted order: {:?}", dice_results);
        
        // Note: Can't sort Vec<Box<dyn Die>> directly since trait objects don't implement Ord
        // You would need to sort by a specific field or implement a custom comparison
        let dice_results = dice
            .iter()
            .map(|d| d.get_current_face_value())
            .collect::<Vec<u32>>();
        println!("Dice results: {:?}", dice_results);
    }
}
