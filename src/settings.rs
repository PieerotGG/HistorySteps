use app::Level;

pub struct Settings<'a> {
    pub level: & 'a mut Box<Level>
}
