use bevy::{math::const_vec2, prelude::*};

pub enum DirectionEnum {
    Up,
    Down,
    Right,
    Left,
    Static,
}

pub const DEFAULT_SIZE: Vec2 = const_vec2!([50.0, 50.0]);

#[derive(Component, Default)]
pub struct Direction {
    pub directions: Vec<DirectionEnum>,
}

#[derive(Component)]
pub struct Player;
