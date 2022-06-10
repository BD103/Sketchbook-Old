use crate::level::{
    data::{MAP_1, MAP_1_META},
    Level, Tile,
};
use crate::palette;
use macroquad::window::screen_width;

pub struct App {
    level: Level,
}

impl App {
    pub fn new() -> Self {
        App {
            level: Level::new(MAP_1, MAP_1_META),
        }
    }

    pub fn draw_level(&self) {
        use macroquad::prelude::*;
        use crate::TILE_RESOLUTION;

        for i in 0..self.level.map.len() {
            draw_rectangle(
                (i % self.level.map_meta.width) as f32 * TILE_RESOLUTION.0
                    + self.level.screen_offset.x,
                (i / self.level.map_meta.height) as f32 * TILE_RESOLUTION.0
                    + self.level.screen_offset.y,
                TILE_RESOLUTION.0,
                TILE_RESOLUTION.0,
                match self.level.map[i] {
                    Tile::Ground => palette::DARK,
                    Tile::Air => palette::LIGHT,
                    Tile::Spikes(_) => palette::PINK,
                    Tile::Flag => palette::BLUE,
                    _ => palette::DARK,
                },
            );
        }
    }
}
