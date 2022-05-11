use bevy::{math::const_vec2, prelude::*};

pub enum DirectionEnum {
    Up,
    Down,
    Right,
    Left,
    Static,
}

impl DirectionEnum {
    pub fn from_code(code: &KeyCode) -> Option<Self> {
        match code {
            KeyCode::Up | KeyCode::W => Some(DirectionEnum::Up),
            KeyCode::Left | KeyCode::A => Some(DirectionEnum::Left),
            KeyCode::Down | KeyCode::S => Some(DirectionEnum::Down),
            KeyCode::Right | KeyCode::D => Some(DirectionEnum::Right),
            _ => None,
        }
    }
}

pub const DEFAULT_SIZE: Vec2 = const_vec2!([50.0, 50.0]);

#[derive(Component, Default)]
pub struct Direction {
    pub directions: Vec<DirectionEnum>,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Camera;
