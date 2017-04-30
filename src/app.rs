use piston_window::*;
use opengl_graphics::GlGraphics;
use piston_window::Button;
use levels::loading::Loading;
use settings::Settings;

pub struct HistorySteps<'a> {
    pub settings: &'a mut Settings
}

impl<'a> HistorySteps<'a> {

    pub fn new(settings: &'a mut Settings) -> HistorySteps<'a> {
        HistorySteps {
            settings: settings
        }
    }

    pub fn level(&mut self) -> &mut Box<Level> {
        &mut (self.settings.level)
    }

}

pub trait Level {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics, settings: &mut Settings);
    fn update(&mut self, args: &UpdateArgs, settings: &mut Settings);
    fn key_press(&mut self, args: &Button, settings: &mut Settings);
}
