//! Gameboard view
use graphics::types::Color;
use graphics::{Context, Graphics};

use GameboardController;

/// settings for the view
pub struct GameboardViewSettings {
	/// Offset from the top-left corner
	pub position: [f64; 2],
	/// Size of teh gameboard
	pub size: f64,
	/// Background Color
	pub background_color: Color,
	/// Border color
	pub border_color: Color,
	/// Edge color (around the whole board)
	pub board_edge_color: Color,
	/// Edge color between the 3x3 sections
	pub section_edge_color: Color,
	/// Edge color between the cells
	pub cell_edge_color: Color,
	/// Edge radius around the whole board
	pub board_edge_radius: f64,
	/// Edge radius around teh sections
	pub section_edge_radius: f64,
	/// Edge radius around the cells
	pub cell_edge_radius: f64,
}

impl GameboardViewSettings {
	/// Creates a  new gameboard view settings
	pub fn new() -> GameboardViewSettings {
		GameboardViewSettings {
			position: [10.0; 2],
			size: 400.0,
			background_color : [0.8, 0.8, 1.0, 1.0],
			border_color : [0.0, 0.0, 0.2, 1.0],
			board_edge_color: [0.0, 0.0, 0.2, 1.0],
			section_edge_color: [0.0, 0.0, 0.2, 1.0],
			cell_edge_color: [0.0, 0.0, 0.2, 1.0],
			board_edge_radius: 3.0,
			section_edge_radius: 2.0,
			cell_edge_radius: 1.0,
		}
	}
}

/// Store visual information about a gameboard
pub struct GameboardView {
	/// Stores gameboard view settings
	pub settings: GameboardViewSettings,
}

impl GameboardView {
	/// Creates a new gameboard view
	pub fn new(settings: GameboardViewSettings) -> GameboardView {
		GameboardView {
			settings: settings,
		}
	}

	/// Draw the gameboard
	pub fn draw<G: Graphics>(&self, c: &Context, controller: &GameboardController, g: &mut G) {

	}
}