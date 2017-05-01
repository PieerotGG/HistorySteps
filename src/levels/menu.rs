use app::Level;
use piston_window::*;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use settings::Settings;
use graphics::*;


pub struct Menu;

impl Level for Menu {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics, settings: &mut Settings) {
        let mut glyphes = GlyphCache::new("assets/FiraSans-Regular.ttf").unwrap();

       gl.draw(args.viewport(), |c, gl| {
           clear([1.0, 1.0, 1.0, 1.0], gl);
           text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
               .draw("Hello world!", &mut glyphes, &c.draw_state, c.transform.trans(10.0, 100.0), gl);
       });
    }

    fn update(&mut self, args: &UpdateArgs, settings: &mut Settings) -> Option<Box<Level>> {
        None
    }

    fn key_press(&mut self, args: &Button, settings: &mut Settings) {
        use piston_window::Button::{Keyboard, Mouse, Controller};
        use piston_window::Key;

        match *args {

            Keyboard(k) => match k {
                Key::Return => println!("Enter"),
                _ => println!("{:?} is a unknow key", k)
            },

            Mouse(c) => match c {
                _ => println!("{:?} is an unknow click", c)
            },

            _ => println!("Unknow input type")

        }
    }
}
