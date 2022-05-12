use bevy::{prelude::*, window::Window};
use rand::Rng;

use crate::components::{Camera, Direction, DirectionEnum, Npc, Player, DEFAULT_SIZE};

pub fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Camera);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(DEFAULT_SIZE),
                ..default()
            },
            transform: offset_x(150.),
            ..default()
        })
        .insert(Direction::default())
        .insert(Player);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(DEFAULT_SIZE),
                ..default()
            },
            transform: offset_x(50.),
            ..default()
        })
        .insert(Npc::default());

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(DEFAULT_SIZE),
                ..default()
            },
            transform: offset_x(-50.),
            ..default()
        })
        .insert(Npc::default());

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(DEFAULT_SIZE),
                ..default()
            },
            transform: offset_x(-150.),
            ..default()
        })
        .insert(Npc::default());
}

enum Axis {
    XPos,
    XNeg,
    YPos,
    YNeg,
}

pub fn sprite_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut player_data: Query<(&mut Direction, &mut Transform), With<Player>>,
    mut sprite_data: Query<(&mut Transform, Option<&Camera>, Option<&mut Npc>), Without<Player>>,
) {
    let win = windows.get_primary().unwrap();

    let width = win.width();
    let height = win.height();

    let player: (Mut<Direction>, Mut<Transform>) = player_data
        .iter_mut()
        .next()
        .expect("could not find player");

    let player_transform = {
        let (mut player_dirs, mut transform) = player;

        let translation: f32 = (if player_dirs.directions.len() > 1 {
            150. / 2.
        } else {
            150.
        }) * time.delta_seconds();

        for direction in player_dirs.directions.iter() {
            let (axis, amount) = match direction {
                DirectionEnum::Up => (Axis::YPos, transform.translation.y + translation),
                DirectionEnum::Down => (Axis::YNeg, transform.translation.y - translation),
                DirectionEnum::Right => (Axis::XPos, transform.translation.x + translation),
                DirectionEnum::Left => (Axis::XNeg, transform.translation.x - translation),
                _ => {
                    continue;
                }
            };

            let is_ok = match axis {
                Axis::XPos => amount < width / 2.,
                Axis::XNeg => amount > -width / 2.,
                Axis::YPos => amount < height / 2.,
                Axis::YNeg => amount > -height / 2.,
            };

            if is_ok {
                match axis {
                    Axis::XPos => transform.translation.x += translation,
                    Axis::XNeg => transform.translation.x -= translation,
                    Axis::YPos => transform.translation.y += translation,
                    Axis::YNeg => transform.translation.y -= translation,
                };
            }
        }

        let mut new_direction: Vec<DirectionEnum> = Vec::new();

        let mut dirs: Vec<DirectionEnum> = keys
            .get_pressed()
            .filter_map(DirectionEnum::from_code)
            .collect();

        new_direction.append(&mut dirs);

        player_dirs.directions = if new_direction.is_empty() {
            vec![DirectionEnum::Static]
        } else {
            new_direction
        };

        *transform
    };

    for (mut transform, cam, npc_opt) in sprite_data.iter_mut() {
        if cam.is_some() {
            continue;
        }

        if let Some(mut npc) = npc_opt {
            let mut rng = rand::thread_rng();
            // This allows me to rig the rand gen in favour of the previous direction
            let mut opts = vec![
                DirectionEnum::Left,
                DirectionEnum::Right,
                DirectionEnum::Up,
                DirectionEnum::Down,
                DirectionEnum::Static,
            ];

            for _ in 0..25 {
                opts.push(npc.previous_direction);
            }

            let direction_index = rng.gen_range(0..opts.len());
            let direction = opts.get(direction_index).unwrap();

            npc.previous_direction = *direction;

            match direction {
                DirectionEnum::Up => {
                    transform.translation.y += 150. * time.delta_seconds();
                }
                DirectionEnum::Left => {
                    transform.translation.x -= 150. * time.delta_seconds();
                }
                DirectionEnum::Down => {
                    transform.translation.y -= 150. * time.delta_seconds();
                }
                DirectionEnum::Right => {
                    transform.translation.x += 150. * time.delta_seconds();
                }
                DirectionEnum::Static => {}
            };
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
