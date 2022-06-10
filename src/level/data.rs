#![allow(unused_imports)]

use super::{
    Tile,
    Tile::{Air as A, Flag as F, Ground as G, Spikes as S},
};
use crate::Direction::{Down as D, Left as L, Right as R, Up as U};
use manyvecs::Vec2f;

/// Contains metadata about the current level.
pub struct MapMeta {
    pub width: usize,
    pub height: usize,
    pub spawn: Vec2f,
    pub command_max: usize,
}

#[rustfmt::skip]
pub const MAP_1: [Tile; 16] = [
    A, A,    A, A,
    A, A,    A, A,
    A, S(L), G, A,
    A, S(L), G, F,
];

pub const MAP_1_META: MapMeta = MapMeta {
    width: 4,
    height: 4,
    spawn: Vec2f { x: 0.0, y: 3.0 },
    command_max: 3,
};
