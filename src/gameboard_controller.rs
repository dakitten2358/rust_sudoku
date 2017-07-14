//! controller for gameboard
use piston::input::GenericEvent;

use Gameboard;

/// Handles events for sudoku gameboard
pub struct GameboardController {
    /// gameboard that we're controlling
    pub gameboard: Gameboard,
    /// selected cell (if any)
    pub selected_cell: Option<[usize; 2]>,
    /// last cursor position
    pub last_cusor_position: [f64; 2],
}

fn is_inside_boundaries(x: f64, y: f64, size: f64) -> bool {
    return x >= 0.0 && x < size && y >= 0.0 && y < size;
}

fn get_selected_cell(x: f64, y: f64, size: f64) -> [usize; 2] {
    let cell_x = ((x / size) * 9.0) as usize;
    let cell_y = ((y / size) * 9.0) as usize;
    [cell_x, cell_y]
}


impl GameboardController {
    /// Creates a new controller for the gameboard
    ///
    /// 	# Aguments
    ///
    /// 	* `gameboard` A gameboard that will represent the state of the game
    ///
    /// # Example
    ///
    /// ```
    /// 	// creating a new gameboard controller
    /// pub use gameboard::Gameboard;
    /// 	pub use gameboard_controller::GameboardController;
    /// 	mod gameboard;
    /// 	mod gameboard_controller;
    ///
    /// 	let gameboard = Gameboard::new();
    /// let controller = GameboardController::new(gameboard);
    /// ```
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController {
            gameboard: gameboard,
            selected_cell: None,
            last_cusor_position: [0.0; 2],
        }
    }

    /// Handles events
    pub fn event<E: GenericEvent>(&mut self, offset: [f64; 2], size: f64, e: &E) {
        use piston::input::{Button, MouseButton};

        // track the mouse position
        if let Some(pos) = e.mouse_cursor_args() {
            self.last_cusor_position = pos;
        }

        // clicked?
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            // find coordinates reklative to offset
            let x = self.last_cusor_position[0] - offset[0];
            let y = self.last_cusor_position[1] - offset[1];

            // make sure it's inside the boundaries, and then  figure out which sell it is
            if is_inside_boundaries(x, y, size) {
                self.selected_cell = Some(get_selected_cell(x, y, size));
            }
        }
    }
}
