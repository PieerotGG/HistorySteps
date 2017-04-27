use piston_window::*;
use opengl_graphics::GlGraphics;
use piston_window::Button;
use levels::loading::Loading;

pub struct HistorySteps {
   pub level: Box<Level>
}

impl HistorySteps {
   
    pub fn new() -> HistorySteps {
        HistorySteps {
           level: Box::new(Loading)
        }
    }

}

pub trait Level {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics);
    fn update(&mut self, args: &UpdateArgs);
    fn key_press(&mut self, args: &Button);
}
