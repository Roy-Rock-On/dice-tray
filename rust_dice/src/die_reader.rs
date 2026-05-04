use crate::die::{Die, DieSummary, RollLog};
use std::collections::HashMap;

///A lightweight tray-facing view into a die owned by the allocator.
///Readers can share the same underlying die by pointing at the same die_id.
#[derive(Clone)]
pub struct DieReader {
	reader_id: usize,
	die_id: usize,
	current_tray_id: usize,
}

impl DieReader {
	///Creates a new die reader for a die in the allocator's die store.
	pub fn new(reader_id: usize, die_id: usize, current_tray_id: usize) -> Self {
		Self {
			reader_id,
			die_id,
			current_tray_id
		}
	}

	///Returns the tray ID this reader is currently assigned to, if any.
	pub fn get_tray_id(&self) -> usize {
		self.current_tray_id
	}

	///Sets or clears the tray this reader is assigned to.
	pub fn set_tray(&mut self, tray_id: usize) {
		self.current_tray_id = tray_id;
	}

	///Returns the reader's unique ID.
	pub fn get_reader_id(&self) -> usize {
		self.reader_id
	}

	///Returns the die ID this reader points to.
	pub fn get_die_id(&self) -> usize {
		self.die_id
	}

	///Builds a summary for this reader from the underlying die.
	pub fn to_summary<'a>(&self, dice: &'a HashMap<usize, Die>) -> Result<DieSummary<'a>, String> {
		let die = dice
			.get(&self.die_id)
			.ok_or_else(|| format!("No die found with ID: {} for reader {}", self.die_id, self.reader_id))?;

		Ok(die.to_summary())
	}

	///Asks the underlying die to roll and returns the resulting roll log.
	pub fn roll(&self, dice: &mut HashMap<usize, Die>) -> Result<RollLog, String> {
		let die = dice
			.get_mut(&self.die_id)
			.ok_or_else(|| format!("No die found with ID: {} for reader {}", self.die_id, self.reader_id))?;

		Ok(die.roll())
	}
}
