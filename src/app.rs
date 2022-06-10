use crate::palette;
use crate::level::{Level, data::{MAP_1, MAP_1_META}};


pub struct App {
    level: Level,
}

impl App {
    pub fn new() -> Self {
        App {
            level: Level::new(MAP_1, MAP_1_META),
        }
    }

    pub fn render(&self) {

    }
}
