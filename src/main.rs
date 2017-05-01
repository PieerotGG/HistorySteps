extern crate piston_window;
extern crate opengl_graphics;
extern crate sdl2_window;
extern crate graphics;

mod app;
mod levels;
mod settings;

use piston_window::*;
use sdl2_window::Sdl2Window;
use opengl_graphics::GlGraphics;
use app::HistorySteps;
use settings::Settings;


fn main() {
    let mut settings: Settings = Settings { };

    let mut window: PistonWindow<Sdl2Window> = WindowSettings::new("HistorySteps", [1280, 720])
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(OpenGL::V3_2);

    let mut level = levels::loading::Loading { time: 0 };
    let mut app = HistorySteps::new(settings, Box::new(level));

    while let Some(e) = window.next() {
        if let Some(ref args) = e.render_args() {
            app.level.render(args, &mut gl, &mut app.settings);
        }

        else if let Some(ref args) = e.update_args() {
            let level = app.level.update(args, &mut app.settings);
            if let Some(level) = level {
                app.level = level;
            }
        }

        else if let Some(ref args) = e.press_args() {
            app.level.key_press(args, &mut app.settings);
        }

    }
}
