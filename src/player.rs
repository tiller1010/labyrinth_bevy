use bevy::{
    math::bounding::Aabb2d,
    prelude::*,
};

use crate::walls::{Wall, wall_collision};
use crate::collider::{Collider, Collision, CollisionEvent};
use crate::finish_area::{FinishArea, finish_area_collision};

const PLAYER_SIZE: Vec2 = Vec2::new(10.0, 10.0);
const PLAYER_COLOR: Color = Color::srgb(50.0, 0.0, 0.0);
const PLAYER_SPEED: f32 = 200.;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(commands: &mut Commands) {
    // Player spawn
    commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(20., 10., 0.),
                    scale: PLAYER_SIZE.extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: PLAYER_COLOR,
                    ..default()
                },
                ..default()
            },
            Player,
            Collider,
    ));
}

pub fn move_player(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    wall_collider_query: Query<&Transform, (With<Wall>, Without<Player>)>,
    finish_area_collider_query: Query<&Transform, (With<FinishArea>, Without<Player>)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let mut player_transform = player_query.single_mut();
    let mut direction_x = 0.0;
    let mut direction_y = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction_x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction_x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction_y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction_y -= 1.0;
    }

    let new_player_position_x = player_transform.translation.x + direction_x * PLAYER_SPEED * time.delta_seconds();
    let new_player_position_y = player_transform.translation.y + direction_y * PLAYER_SPEED * time.delta_seconds();

    player_transform.translation.x = new_player_position_x;
    player_transform.translation.y = new_player_position_y;
    // End move player

    // Start Wall Collision Detection
    let player_bounding_box = Aabb2d::new(
        player_transform.translation.truncate(), 
        player_transform.scale.truncate() / 2.,
    );

    for wall_transform in &wall_collider_query {
        let collision = wall_collision(
            &player_bounding_box,
            Aabb2d::new(
                wall_transform.translation.truncate(),
                wall_transform.scale.truncate() / 2.,
            ),
        );

        if let Some(collision) = collision {
            collision_events.send_default();

            let mut reflect_x = 0.;
            let mut reflect_y = 0.;

            match collision {
                Collision::Left => reflect_x = -7.9,
                Collision::Right => reflect_x = 7.9,
                Collision::Top => reflect_y = 7.9,
                Collision::Bottom => reflect_y = -7.9,
            }

            if reflect_x != 0. {
                player_transform.translation.x = player_transform.translation.x + reflect_x;
            }

            if reflect_y != 0. {
                player_transform.translation.y = player_transform.translation.y + reflect_y;
            }
        }
    }
    // End Wall Collision Detection

    // Start finish area collision detection
    finish_area_collision(&mut commands, &player_bounding_box, &finish_area_collider_query);
}


