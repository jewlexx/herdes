use bevy::{prelude::*, DefaultPlugins};

#[macro_use]
extern crate lazy_static;

enum DirectionEnum {
    Up,
    Down,
    Right,
    Left,
    Static,
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum SpriteType {
    Person,
    Player,
}

lazy_static! {
    static ref DEFAULT_SIZE: Vec2 = Vec2::new(50.0, 50.0);
}

#[derive(Component, Default)]
struct Direction {
    directions: Vec<DirectionEnum>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .run();
    println!("Hello, world!");
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(*DEFAULT_SIZE),
                ..default()
            },
            transform: offset_x(150.),
            ..default()
        })
        .insert(Direction::default())
        .insert(SpriteType::Player);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(*DEFAULT_SIZE),
                ..default()
            },
            transform: offset_x(50.),
            ..default()
        })
        .insert(Direction::default())
        .insert(SpriteType::Person);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(*DEFAULT_SIZE),
                ..default()
            },
            transform: offset_x(-50.),
            ..default()
        })
        .insert(Direction::default())
        .insert(SpriteType::Person);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(*DEFAULT_SIZE),
                ..default()
            },
            transform: offset_x(-150.),
            ..default()
        })
        .insert(Direction::default())
        .insert(SpriteType::Person);
}

fn sprite_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    sprite_data: Query<(&mut Direction, &mut Transform, &SpriteType)>,
) {
    let mut sprites_query = sprite_data;

    let player = sprites_query
        .iter_mut()
        .find(|(_, _, sprite_type)| {
            let t = **sprite_type;
            t == SpriteType::Player
        })
        .expect("could not find player");

    let player_transform = {
        let (mut logo, mut transform, _) = player;

        let translation: f32 = (if logo.directions.len() > 1 {
            150. / 2.
        } else {
            150.
        }) * time.delta_seconds();

        for direction in logo.directions.iter() {
            match direction {
                DirectionEnum::Up => transform.translation.y += translation,
                DirectionEnum::Down => transform.translation.y -= translation,
                DirectionEnum::Right => transform.translation.x += translation,
                DirectionEnum::Left => transform.translation.x -= translation,
                _ => {}
            }
        }

        let mut new_direction: Vec<DirectionEnum> = Vec::new();

        if keys.pressed(KeyCode::Up) {
            new_direction.push(DirectionEnum::Up);
        }
        if keys.pressed(KeyCode::Down) {
            new_direction.push(DirectionEnum::Down);
        }
        if keys.pressed(KeyCode::Right) {
            new_direction.push(DirectionEnum::Right);
        }
        if keys.pressed(KeyCode::Left) {
            new_direction.push(DirectionEnum::Left);
        }

        logo.directions = if new_direction.is_empty() {
            vec![DirectionEnum::Static]
        } else {
            new_direction
        };

        *transform
    };

    for sprite in sprites_query.iter_mut() {
        let (_, mut transform, sprite_type) = sprite;

        if *sprite_type == SpriteType::Player {
            continue;
        }

        let distance_from = in_range(transform.as_ref(), &player_transform, 150.);
        if distance_from.0 {
            let to_move_x = distance_from.1;
            let to_move_y = distance_from.2;

            transform.translation.x += to_move_x * time.delta_seconds();
            transform.translation.y += to_move_y * time.delta_seconds();
        }
    }
}

fn offset_x(x: f32) -> Transform {
    Transform::from_xyz(x, 0., 0.)
}

fn in_range(a: &Transform, b: &Transform, range: f32) -> (bool, f32, f32) {
    let a_x = a.translation.x;
    let a_y = a.translation.y;
    let b_x = b.translation.x;
    let b_y = b.translation.y;

    let x_dist = a_x - b_x;
    let y_dist = a_y - b_y;
    let distance = (x_dist.powi(2) + y_dist.powi(2)).sqrt();

    (distance < range, x_dist, y_dist)
}
