use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
    DefaultPlugins,
};

#[macro_use]
extern crate lazy_static;

mod components;
mod config;
mod errors;
mod systems;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Herd Simulation".to_string(),
            present_mode: PresentMode::Fifo,
            mode: WindowMode::Fullscreen,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(systems::setup)
        .add_system(systems::sprite_movement)
        .run();
}
