use crate::Direction;

/// Represents a tile in a [`Level`](super::Level).
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum Tile {
    Ground,
    Air,
    Spikes(Direction),
    Flag,
}
