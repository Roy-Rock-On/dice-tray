use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::cmp::Ordering;
use std::result;

/// Used to request specific result types from a Die roll.
#[derive(Debug, Clone)]
pub enum DieResultType{
    Number
}

/// Used to return specific result types from a Die roll and wraps the returned value.
#[derive(Debug, Clone)] 
pub enum DieResult{
    Number(u32)
}


/// Represents a physical dice. Includes a string identifier, it's own SmallRng seed, ability to roll and compare rolls. 
#[derive(Debug, Clone)]
pub struct Die{
    rng: SmallRng,
    identity: String,
    faces: u32,
    current_face: u32,
    result_type : DieResultType
}

impl Die {
    /// Creates a new die, with an optional string identifier. 
    /// If no identity is provided will default to the dice notation number (e.g. 'd6', 'd100').
    /// The new dice is rolled on creation to give it a random self_current face. 
    pub fn new(identity: Option<String>, result_type: Option<DieResultType>, faces: u32) -> Self {
        let mut new_die = Die {
            rng : SmallRng::from_rng(&mut rand::rng()),
            identity: identity.unwrap_or_else(|| "d".to_string() + &faces.to_string()),
            faces,
            current_face: 1,
            result_type: result_type.unwrap_or(DieResultType::Number)     
        };
        new_die.roll();
        new_die
    }

    /// Returns the ID of the die as a string slice.
    pub fn get_id(&self) -> &str {
        &self.identity
    }

    /// Returns the result of the die roll based on the result type of the die.
    pub fn get_result(&self) -> DieResult {
        self.get_result_as(&self.result_type)
    }

    /// Returns the result of the die roll based on the provided result type.
    pub fn get_result_as(&self, result_type : &DieResultType) -> DieResult{
        match result_type{
            DieResultType::Number => DieResult::Number(self.current_face)
        }
    }

    /// Rolls the dice to give it a random number between 1 and self.faces (inclusive).
    pub fn roll(&mut self) {
        self.current_face = self.rng.random_range(1..=self.faces);
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

    pub fn set_identity(&mut self, identity: String) {
        self.identity = identity;
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
    fn clamp_to_bounds(&self, value : u32) -> u32 {
        if value < 1 {
            1
        } else if value > self.faces {
            self.faces
        } else {
            value
        }
    }
}

/// Creates a vector of dice with a count equal to conunt and a number of faces equal to faces.
pub fn new_dice_set(count : u32, faces : u32) -> Vec<Die>{
    let mut new_die_vec = Vec::new();
    for _i in 0..count{
        let new_die = Die::new(None, None, faces);
        new_die_vec.push(new_die);
    }
    new_die_vec
}

/// Creates a vector of dice from a given vector of u32. Each die has a face count equal each u32 in the count. 
pub fn new_dice_from_vec(dice : Vec<u32>) -> Vec<Die>{
    dice.iter().map(|f| Die::new(None, None, *f)).collect()
}

pub fn new_dice_from_vec_with_id(dice : Vec<(Option<String>, u32)>) -> Vec<Die>{
    dice.iter().map(|i| Die::new(i.0.clone(), None, i.1)).collect()
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
        (self.current_face, self.faces, &self.identity)
            .cmp(&(other.current_face, other.faces, &other.identity))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_die_creation_and_roll() {
        let faces = 20;
        let mut die = Die::new(Some("TestDie".to_string()), None, faces);
        assert_eq!(die.get_identity(), "TestDie");
        assert!(die.get_current_face() >= 1 && die.get_current_face() <= faces);
        let first_roll = die.get_current_face();
        die.roll();
        println!("First roll: {}, Second roll: {}", first_roll, die.get_current_face());
    }

    #[test]
    fn dice_ordering() {
        let die1 = Die::new(None, None, 6);
        let die2 = Die::new(None, None, 6);
        let die3 = Die::new(None, None, 6);
        let die4 = Die::new(None, None, 6);
        let die5 = Die::new(None, None, 6);
        let die6 = Die::new(None, None, 6);
        
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
        let dice_results = dice.iter().map(|d| d.get_current_face()).collect::<Vec<u32>>();
        println!("Dice in sorted order: {:?}", dice_results);
    }
}