//! This is a rust-based CLI dice roller, and library to roll your own virtual dice in rust.
//! The goal is to mimic real physical dice as much as possible. 
//! Dice in this crate have thier own internal rng seeds, and can be called out with a string idenity. 

///Library for dice and dice functions. Each die has its own rng seed and a set face count, determined at construction.
pub mod dice;

///Library for managing a tray of dice. A tray can hold multiple dice, roll them all, remove them, clear itself, and more.
pub mod tray;

///Library for logging the state of a tray/dice to the console.
pub mod logger;

///Library for parsing commands passed from the CLI.
pub mod cli_parser;

/// Library for creating roll tables and looking up dice results on said tables. 
pub mod tables;

/// Library for managing user settings including result tables, dice profiles, and more.
pub mod settings;