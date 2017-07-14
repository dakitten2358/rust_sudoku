//! controller for gameboard
use piston::input::GenericEvent;

use Gameboard;

/// Handles events for sudoku gameboard
pub struct GameboardController {
	/// gameboard that we're controlling
	pub gameboard: Gameboard,
}

impl GameboardController {
	/// Creates a new controller for the gameboard
	///
	///	# Aguments
	///
	///	* `gameboard` A gameboard that will represent the state of the game
	///
	/// # Example
	///
	/// ```
	///	// creating a new gameboard controller
	/// pub use gameboard::Gameboard;
	///	pub use gameboard_controller::GameboardController;
	///	mod gameboard;
	///	mod gameboard_controller;
	/// 
	///	let gameboard = Gameboard::new();
	/// let controller = GameboardController::new(gamebocfard);
	/// ```
	pub fn new(gameboard: Gameboard) -> GameboardController {
		GameboardController { 
			gameboard: gameboard,
		}
	}

	/// Handles events
	pub fn event<E: GenericEvent>(&mut self, e: &E) {

	}
}