use bevy::{prelude::*, DefaultPlugins};

mod components;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(systems::setup)
        .add_system(systems::sprite_movement)
        .run();
    println!("Hello, world!");
}
