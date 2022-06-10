use super::data::MapMeta;
use super::Tile;
use crate::Direction;

use manyvecs::Vec2f;

const FRICTION: f32 = 0.2;

pub struct Level {
    pub map: Vec<Tile>,
    pub map_meta: MapMeta,
    pub gravity: Direction,

    pub player_pos: Vec2f,
    // player_delta: Vec2f,
}

impl Level {
    pub fn new<const N: usize>(map: [Tile; N], meta: MapMeta) -> Self {
        // Copy bytes before moving meta
        let player_pos = meta.spawn;

        Level {
            map: Vec::from(map),
            map_meta: meta,
            gravity: Direction::Down,

            player_pos,
            // player_delta: Vec2f::new(0.0, 0.0),
        }
    }

    // Level functions
    pub fn change_gravity(&mut self, gravity: Direction) {
        self.gravity = gravity;
    }

    pub fn update(&mut self) {
        self.move_player();
    }

    // Map functions
    pub fn map_tile(&self, x: usize, y: usize) -> Tile {
        self.map[x + y * self.map_meta.height]
    }

    // Player functions
    fn move_player(&mut self) {
        match self.gravity {
            Direction::Down => self.player_pos.y -= 0.5,
            Direction::Up => self.player_pos.y += 0.5,
            Direction::Left => self.player_pos.x -= 0.5,
            Direction::Right => self.player_pos.x += 0.5,
        }
    }
}
