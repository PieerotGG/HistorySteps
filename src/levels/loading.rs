use super::super::app::Level;
use piston_window::*;
use opengl_graphics::GlGraphics;


pub struct Loading;

impl Level for Loading {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {

    }

    fn update(&mut self, args: &UpdateArgs) {

    }

    fn key_press(&mut self, args: &Button) {
        use piston_window::Button::{Keyboard, Mouse, Controller};

        match *args {
            
            Keyboard(k) => match k {
                _ => println!("{:?} is a unknow key", k)
            },

            Mouse(c) => match c {
                _ => println!("{:?} is an unknow click", c)
            },

            _ => println!("Unknow input type")

        }
    }
}
