extern crate piston;
extern crate glutin_window;

use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::window::WindowSettings;
use glutin_window::GlutinWindow;

fn main() {
	let settings = WindowSettings::new("Sudoku", [512;2]).exit_on_esc(true);
	let mut window: GlutinWindow = settings.build().expect("failed to create window");
	let mut events = Events::new(EventSettings::new().lazy(true));
	while let Some(e) = events.next(&mut window) {

	}
}
