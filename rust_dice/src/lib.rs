//! This is a rust-based CLI dice roller, and library to roll your own virtual dice in rust.
//! The goal is to mimic real physical dice as much as possible.
//! Dice in this crate have thier own internal rng seeds, and can be called out with a string idenity.

///Module for dice and dice functions. Each die has its own rng seed and a set face count, determined at construction.
pub mod dice;

///Module for data classes that allow fore saving and loading dice using the serde crate.
///This is required becasue custom dice may include types that don't implement the Serialize/Deserialize Trait.
///And because the dice/tray ids need to be reassinged by the dice allocator at runtime.    
pub mod dice_data;

///Module  used for allocating dice to an app. The DiceAllocator trait is used to assign dice unique ID numbers and connect settings to the dice tray.
pub mod dice_allocator;

///Helper functions to quickly build sets of dice.
pub mod dice_builders;

//Dice profiles are used to build new dice. Dice profiles can be saved/loaded from JSON (not yet implemented)
pub mod dice_profile;

///Module  for managing a tray of dice. A tray can hold multiple dice, roll them all, remove them, clear itself, and more.
pub mod tray;

///Module  for creating roll tables and looking up dice results on said tables.
pub mod tables;

///Module for managing user settings including result tables, dice profiles, and more.
pub mod settings;

