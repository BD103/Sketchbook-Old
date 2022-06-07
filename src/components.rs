use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Position(Vec2);

#[derive(Component)]
pub enum Gravity {
    Down,
    Up,
    Left,
    Right,
}
