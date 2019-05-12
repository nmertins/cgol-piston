extern crate cgol;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod view;
mod controller;

use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

use glutin_window::GlutinWindow;

use opengl_graphics::{GlGraphics, OpenGL};

use view::{CgolView, CgolViewSettings};
use crate::controller::CgolController;
use cgol::{GameOfLife, GameOfLifeSettings, GameError};

fn main() {
    let opengl = OpenGL::V3_2;
    let window_settings = WindowSettings::new("Game of Life", [700, 700])
        .opengl(opengl)
        .exit_on_esc(true);
    let mut window: GlutinWindow = window_settings.build().expect("Could not create window");
    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let game_settings = GameOfLifeSettings::from_file("resources/initial.state").unwrap();

    let (rows, _) = game_settings.get_dimensions();
    let view_settings = CgolViewSettings::new()
        .cells_per_row(rows);
    let view = CgolView::new(view_settings);
    let mut controller = CgolController{};

    let mut game = GameOfLife::new(game_settings);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            controller.event(&e);
            gl.draw(args.viewport(), |c, g| {
                view.draw(game.get_state(), &c, g);
            });
        }
    }
}
