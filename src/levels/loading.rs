use app::Level;
use piston_window::*;
use opengl_graphics::GlGraphics;
use settings::Settings;
use levels::menu::Menu;

pub struct Loading {
    pub time: i32
}

impl Level for Loading {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics, settings: &mut Settings) {
    }

    fn update(&mut self, args: &UpdateArgs, settings: &mut Settings) -> Option<Box<Level>> {
        self.time += 1;
        println!("{}", self.time);
        if self.time == 1000 {
            Some(Box::new(Menu))
        } else {
            None
        }

    }

    fn key_press(&mut self, args: &Button, settings: &mut Settings) {
        use piston_window::Button::{Keyboard, Mouse, Controller};
        use piston_window::Key;

        match *args {

            Keyboard(k) => match k {
                Key::Space => println!("Space"),
                _ => println!("{:?} is a unknow key", k)
            },

            Mouse(c) => match c {
                _ => println!("{:?} is an unknow click", c)
            },

            _ => println!("Unknow input type")

        }
    }
}
