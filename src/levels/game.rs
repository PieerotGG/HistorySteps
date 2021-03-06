use app::Level;
use piston_window::*;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use settings::Settings;
use graphics::*;


pub struct Game {
    pub x: f64,
    pub y: f64,
}

impl Level for Game {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics, settings: &mut Settings) {
        let mut glyphes = GlyphCache::new("assets/FiraSans-Regular.ttf").unwrap();

        gl.draw(args.viewport(), |c, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);
            Rectangle::new([1.0, 1.0, 1.0, 1.0]).draw([1.0, 1.0, 20.0, 10.0],
                                                      &c.draw_state,
                                                      c.transform.trans(self.x, self.y),
                                                      gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs, settings: &mut Settings) -> Option<Box<Level>> {
        None
    }

    fn key_press(&mut self, args: &Button, settings: &mut Settings) -> Option<Box<Level>> {
        use piston_window::Button::{Keyboard, Mouse, Controller};
        use piston_window::Key;

        match *args {

            Keyboard(k) => {
                match k {
                    Key::D => self.x += 5.0,
                    Key::Q => self.x -= 5.0,
                    Key::Z => self.y -= 5.0,
                    Key::S => self.y += 5.0,
                    _ => println!("Unknow key: {:?}", k),
                }
            }

            Mouse(c) => {
                match c {
                    _ => println!("{:?} is an unknow click", c),
                }
            }

            _ => println!("Unknow input type"),

        }
        None
    }
}
