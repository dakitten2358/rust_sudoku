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

    fn is_valid(&self, ind:[usize; 2], val:u8) -> bool {
        /// check to see if we already have this number in the given row or column
        for i in 0..9 {
            if (self.cells[i][ind[0]] == val) {
                return false;
            }
            if (self.cells[ind[1]][i] == val) {
                return false;
            }
        }

        // check the remaining four spaces (tl, tr, bl, br) in this section
        let section_row = 3 * (ind[1] / 3);
        let section_col = 3 * (ind[0] / 3);

        let row1 = (ind[1]+2) % 3;
        let row2 = (ind[1]+4) % 3;

        let col1 = (ind[0]+2) % 3;
        let col2 = (ind[0]+4) % 3;

        if (self.cells[row1 + section_row][col1 + section_col] == val) {
            return false;
        }
        if (self.cells[row2 + section_row][col1 + section_col] == val) {
            return false;
        }

        if (self.cells[row1 + section_row][col2 + section_col] == val) {
            return false;
        }
        if (self.cells[row2 + section_row][col2 + section_col] == val) {
            return false;
        }


        true
    }

    /// solve a gameboard
    pub fn solve(&mut self, ind:[usize; 2]) -> bool {
        // make sure we exit if we're done
        if SIZE == ind[1] {
            return true;
        }

        // skip cells that already have a value
        if self.cells[ind[1]][ind[0]] > 0 {
            if ind[0] == (SIZE-1) {
                if self.solve([0, ind[1] + 1]) {
                    return true;
                }
            }
            else {
                if self.solve([ind[0] + 1, ind[1]]) {
                    return true;
                }
            }
        }

        for next_val in 1..10 {
            if self.is_valid(ind, next_val) {
                self.set_cell(ind, next_val);

                if ind[0] == (SIZE-1) {
                    if self.solve([0, ind[1] + 1]) {
                        return true;
                    }
                }
                else {
                    if self.solve([ind[0] + 1, ind[1]]) {
                        return true;
                    }
                }

                self.set_cell(ind, 0);
            }
        }

        false
    }
}
