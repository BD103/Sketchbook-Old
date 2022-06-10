use crate::level::{
    data::{MAP_1, MAP_1_META},
    Level, Tile,
};
use crate::palette;

/// App context that controls the majority of the game.
pub struct App {
    level: Level,
}

impl App {
    /// Creates a new app context.
    pub fn new() -> Self {
        App {
            level: Level::new(MAP_1, MAP_1_META),
        }
    }

    pub fn update(&mut self) {
        self.level.update();
    }

    /// Loads a new level to be drawn.
    pub fn load_level(&mut self, level: Level) {
        self.level = level;
    }

    /// Draws the current level to the screen.
    pub fn draw_level(&self) {
        use crate::TILE_RESOLUTION;
        use macroquad::prelude::*;

        draw_rectangle(self.level.screen_offset.x, self.level.screen_offset.y, (self.level.map_meta.width * TILE_RESOLUTION.1) as f32, (self.level.map_meta.height * TILE_RESOLUTION.1) as f32, palette::LIGHT);

        for i in 0..self.level.map.len() {
            draw_rectangle(
                (i % self.level.map_meta.width) as f32 * TILE_RESOLUTION.0
                    + self.level.screen_offset.x,
                (i / self.level.map_meta.height) as f32 * TILE_RESOLUTION.0
                    + self.level.screen_offset.y,
                TILE_RESOLUTION.0,
                TILE_RESOLUTION.0,

                #[allow(unreachable_patterns)]
                match self.level.map[i] {
                    Tile::Ground => palette::DARK,
                    Tile::Air => palette::TRANSPARENT,
                    Tile::Spikes(_) => palette::PINK,
                    Tile::Flag => palette::BLUE,
                    _ => palette::DARK,
                },
            );
        }
    }
}
