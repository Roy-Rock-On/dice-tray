//! This is a rust-based persistant dice roller virtual dice in rust.
//! The goal is to mimic real physical dice as much as possible.
//! Dice in this crate have thier own internal rng and variation in face weights.
//! These dice are meant to be just a tiny bit unfair. They have personality and persistance.
//! They serialize to JSON and save between sessions, so you can decide which dice youy like best in your virtual dice bag.

///New module for building the dice again from the ground up with it's own internal RNG.
mod die;

///Die trays sort die readers and hold an internal list of Die reader Ids.
mod die_tray;

///Die readers provide tray-facing views into shared dice in the allocator.
mod die_reader;

///Used to generate and track unique IDs for variuous components. For binding to a frontend UI.
mod id_generator;

///Die allocators create dice and assign them to a has set with unique IDs.
///Responsible for sorting dice into trays, updating them, and passing RollLogs to a user interface. 
///Used to integrate a persistant dice tray with web or CLI applications.
pub mod die_allocator;


