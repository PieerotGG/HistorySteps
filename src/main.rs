extern crate piston_window;
extern crate opengl_graphics;
extern crate sdl2_window;

mod app;
mod levels;
mod settings;

use piston_window::*;
use sdl2_window::Sdl2Window;
use opengl_graphics::GlGraphics;
use app::HistorySteps;
use settings::Settings;


fn main() {
    let mut settings: Settings = Settings { level: Box::new(levels::loading::Loading { time: 0 })};

    let mut window: PistonWindow<Sdl2Window> = WindowSettings::new("HistorySteps", [500, 200])
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(OpenGL::V3_2);

    let mut app = HistorySteps::new(&mut settings);

    while let Some(e) = window.next() {
       if let Some(ref args) = e.render_args() {
           app.level().render(args, &mut gl, app.settings);
       }

       if let Some(ref args) = e.update_args() {
           app.level().update(args);
       }

       if let Some(ref args) = e.press_args() {
           app.level().key_press(args);
       }

    }
}
