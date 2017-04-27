extern crate piston_window;
extern crate opengl_graphics;
extern crate sdl2_window;

mod app;
mod levels;

use piston_window::*;
use sdl2_window::Sdl2Window;
use opengl_graphics::GlGraphics;
use app::HistorySteps;


fn main() {
    let mut window: PistonWindow<Sdl2Window> = WindowSettings::new("HistorySteps", [500, 200])
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(OpenGL::V3_2);

    let mut app = HistorySteps::new();

    while let Some(e) = window.next() {
       if let Some(ref args) = e.render_args() {
           app.level.render(args, &mut gl);
       }

       if let Some(ref args) = e.update_args() {
           app.level.update(args);
       }

       if let Some(ref args) = e.press_args() {
           app.level.key_press(args);
       }

    }
}
