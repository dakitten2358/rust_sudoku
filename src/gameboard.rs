//! Game board logic

/// Size of the game board
const SIZE: usize = 9;

/// Actual gameboard
pub struct Gameboard {
	/// Stores the content of the cells - 0 for empty
	pub cells: [[u8; SIZE]; SIZE],
}

impl Gameboard {
	/// Creates a new gameboard
	pub fn new() -> Gameboard {
		Gameboard {
			cells: [[0; SIZE]; SIZE],
		}
	}
}