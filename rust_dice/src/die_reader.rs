use serde::{Deserialize, Serialize};
use crate::die::{Die, DieResult};

///A lightweight tray-facing view into a die owned by the allocator.
///Readers can share the same underlying die by pointing at the same die_id.
#[derive(Clone, Serialize, Deserialize)]
pub struct DieReader {
	die_id: usize,
    reader_id: usize,
    total_faces: u32,
    die_label: String, 
    current_face: u32,
    current_result: DieResult
}

impl DieReader {
	///Creates a new die reader for a die in the allocator's die store.
	pub fn new(die: &Die, reader_id: usize) -> Self {
		Self {
			die_id: die.get_id(),
            reader_id,
            total_faces: die.get_face_count(),
            die_label: die.get_label().to_string(),
            current_face: die.get_current_face(),
            current_result: die.get_current_result().clone()
		}
	}

	///Returns the die ID this reader points to.
	pub fn get_die_id(&self) -> usize {
		self.die_id
	}

    pub fn get_reader_id(&self) -> usize{
        self.reader_id
    }

    pub fn get_label(&self) -> &str{
        &self.die_label
    }

    pub fn get_face_count(&self) -> u32 {
        self.total_faces
    }

    pub fn get_current_face(&self) -> u32{
        self.current_face
    }

    pub fn get_current_result(&self) -> &DieResult{
        &self.current_result
    }

	///Asks the provided die to roll, update the reader and return a roll log- or error.
	pub fn roll(&mut self, die: &mut Die) {
		let log = die.roll();
        self.current_face = die.get_current_face();
        self.current_result = die.get_current_result().clone();
	}
}

impl PartialEq for DieReader {
    fn eq(&self, other: &Self) -> bool {
        self.total_faces == other.total_faces
            && self.current_face == other.current_face
            && self.current_result.get_num().unwrap_or(0) == other.current_result.get_num().unwrap_or(0)
            && self.die_id == other.die_id
    }
}

impl Eq for DieReader {}

impl<'a> PartialOrd for DieReader {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DieReader {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_faces
            .cmp(&other.total_faces)
            .then_with(|| self.current_face.cmp(&other.current_face))
            .then_with(|| self.current_result.get_num().unwrap_or(0).cmp(&other.current_result.get_num().unwrap_or(0)))
            .then_with(|| self.die_id.cmp(&other.die_id))
    }
}
