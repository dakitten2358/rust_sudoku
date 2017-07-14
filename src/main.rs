#![deny(missing_docs)]

//! Rust Sudoku game.

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics, Filter, TextureSettings};
use opengl_graphics::glyph_cache::GlyphCache;

pub use gameboard::Gameboard;
pub use gameboard_controller::GameboardController;
pub use gameboard_view::{GameboardView, GameboardViewSettings};

mod gameboard;
mod gameboard_controller;
mod gameboard_view;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Sudoku", [512; 2]).opengl(opengl).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("failed to create window");
    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let gameboard = Gameboard::new();
    let mut gameboard_controller = GameboardController::new(gameboard);
    let gameboard_view_settings = GameboardViewSettings::new();
    let gameboard_view = GameboardView::new(gameboard_view_settings);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", texture_settings)
        .expect("failed to load font");

    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(gameboard_view.settings.offset,
                                   gameboard_view.settings.size,
                                   &e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                clear([100.0 / 255.0, 149.0 / 255.0, 237.0 / 255.0, 1.0], g);
                gameboard_view.draw(&c, &gameboard_controller, g, glyphs);
            });
        }
    }
}
