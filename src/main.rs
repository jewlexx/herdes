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

use config::Config;

fn main() {
    let config = Config::init().unwrap();
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Herd Simulation".to_string(),
            present_mode: PresentMode::Fifo,
            mode: WindowMode::Windowed,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(systems::setup)
        .add_system(systems::sprite_movement)
        .run();
}
