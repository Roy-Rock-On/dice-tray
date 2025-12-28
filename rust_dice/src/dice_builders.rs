use crate::dice::{Die, Die32};
use crate::dice_profile::{DieProfile, DieType};

///Creates a new die from a die profile.
pub fn new_die(id: usize, profile : &DieProfile) -> impl Die + use<> {
    match profile.die_type {
        DieType::Numerical(faces) =>{
            Die32::new(id, profile.label.clone(), faces)
        },
        DieType::Custom => {
            todo!("Implement custome dice later.")
        }
    }
}

/// Creates a new numerical die with an optional label. Returns a box to the trait. 
pub fn new_num_die(id : usize, label : Option<String>, faces: u32) -> Box<dyn Die>{
    let profile = DieProfile::new(label, DieType::Numerical(faces));
    Box::new(new_die(id, &profile))
}

/// Creates a vector of dice with a count equal to conunt and a number of faces equal to faces.
pub fn new_dice_set(count: usize, faces: u32) -> Vec<Box<dyn Die>> {
    let mut new_die_vec = Vec::new();
    let profile = DieProfile::new(None, DieType::Numerical(faces));
    for i in 0..count {
        let new_die: Box<dyn Die> = Box::new(new_die(i, &profile));
        new_die_vec.push(new_die);
    }
    new_die_vec
}

/// Creates a vector of dice from a given vector of u32. Each die has a face count equal each u32 in the count.
pub fn new_dice_from_vec(dice: Vec<u32>) -> Vec<Box<dyn Die>> {
    dice.iter().enumerate().map(|(i, f)| {
        let profile = DieProfile::new(None, DieType::Numerical(*f));
        Box::new(new_die(i, &profile)) as Box<dyn Die>
    }).collect()
}

pub fn new_dice_from_vec_with_label(dice: Vec<(Option<String>, u32)>) -> Vec<Box<dyn Die>> {
    dice.iter().enumerate().map(|(index, (label, faces))| {
        let profile = DieProfile::new(label.clone(), DieType::Numerical(*faces));
        Box::new(new_die(index, &profile)) as Box<dyn Die>
    }).collect()
}