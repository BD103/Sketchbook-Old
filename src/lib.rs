mod app;
mod level;

pub mod palette;

pub use self::app::App;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}
