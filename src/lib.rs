mod app;
mod level;

pub mod palette;

pub use self::app::App;

pub const SCREEN_WIDTH: i32 = 1000;
pub const SCREEN_HEIGHT: i32 = 600;
pub const TILE_RESOLUTION: (f32, usize) = (64.0, 64);

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}
