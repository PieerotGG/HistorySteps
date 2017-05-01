use piston_window::*;
use opengl_graphics::GlGraphics;
use piston_window::Button;
use levels::loading::Loading;
use settings::Settings;

pub struct HistorySteps {
    pub settings: Settings,
    pub level: Box<Level>
}

impl HistorySteps {

    pub fn new(settings: Settings, level: Box<Level>) -> HistorySteps {
        HistorySteps {
            settings: settings,
            level: level
        }
    }
}

pub trait Level {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics, settings: &mut Settings);
    fn update(&mut self, args: &UpdateArgs, settings: &mut Settings) -> Option<Box<Level>>;
    fn key_press(&mut self, args: &Button, settings: &mut Settings);
}
