use crate::Direction;

#[derive(Copy, Clone, Debug)]
pub enum Tile {
    Ground,
    Air,
    Spikes(Direction),
    Flag,
}
