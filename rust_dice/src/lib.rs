//! This is a rust-based persistent dice roller virtual dice in rust.
//! The goal is to mimic real physical dice as much as possible.
//! Dice in this crate have their own internal rng and variation in face weights.
//! These dice are meant to be just a tiny bit unfair. They have personality and persistence.
//! They serialize to JSON and save between sessions, so you can decide which dice you like best in your virtual dice bag.

///New module for building the dice again from the ground up with it's own internal RNG.
pub mod die;

///Holds various data types used for moving dice information to different parts of this application.
pub mod die_data;

///Die trays sort die readers and hold an internal list of Die reader Ids.
pub mod die_tray;

///Die readers provide tray-facing views into shared dice in the allocator.
pub mod die_reader;

///Used to generate and track unique IDs for various components. For binding to a frontend UI.
pub mod id_generator;

///Used to target dice context dependently by tray or at the allocator level.
pub mod die_targets;

///Die allocators create dice and assign them to a has set with unique IDs.
///Responsible for sorting dice into trays, updating them, and passing RollLogs to a user interface. 
///Used to integrate a persistent dice tray with web or CLI applications.
pub mod die_allocator;


