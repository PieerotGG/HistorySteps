use piston_window::*;
use opengl_graphics::GlGraphics;
use piston_window::Button;
use levels::loading::Loading;
use settings::Settings;

pub struct HistorySteps<'a> {
    pub settings: & 'a Settings<'a>
}

impl<'a> HistorySteps<'a> {

    pub fn new(settings: &Settings) -> HistorySteps<'a> {
        HistorySteps {
            settings: settings
        }
    }

    pub fn level(&self) -> Box<Level> {
       self.settings.level 
    }

}

pub trait Level {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics, settings: &Settings);
    fn update(&mut self, args: &UpdateArgs);
    fn key_press(&mut self, args: &Button);
}
