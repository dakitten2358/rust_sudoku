extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};

fn main() {
	let opengl = OpenGL::V3_2;
	let settings = WindowSettings::new("Sudoku", [512;2]).opengl(opengl).exit_on_esc(true);
	let mut window: GlutinWindow = settings.build().expect("failed to create window");
	let mut events = Events::new(EventSettings::new().lazy(true));
	let mut gl = GlGraphics::new(opengl);
	while let Some(e) = events.next(&mut window) {
		if let Some(args) = e.render_args() {
			gl.draw(args.viewport(), |c, g| {
				use graphics::{clear};

				clear([100.0/255.0, 149.0/255.0, 237.0/255.0, 1.0], g);
			});
		}
	}
}
