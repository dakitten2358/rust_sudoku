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
        Gameboard { cells: [[0; SIZE]; SIZE] }
    }

    /// Gets the character to display at a given cell
    pub fn display_character_at(&self, ind: [usize; 2]) -> Option<char> {
        Some(match self.cells[ind[1]][ind[0]] {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            _ => return None,
        })
    }

    /// sets the value of a cell
    pub fn set_cell(&mut self, ind: [usize; 2], val: u8) {
        self.cells[ind[1]][ind[0]] = val;
    }
}
