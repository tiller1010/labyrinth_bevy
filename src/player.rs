use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use std::time::Duration;

use crate::walls::{Wall, wall_collision};
use crate::collider::{Collider, Collision, CollisionEvent};
use crate::finish_area::{FinishArea, finish_area_collision};
use crate::enemy::Enemy;

const PLAYER_SIZE: Vec2 = Vec2::new(10.0, 10.0);
const PLAYER_COLOR: Color = Color::srgb(50.0, 0.0, 0.0);
const PLAYER_SPEED: f32 = 200.;
const PLAYER_ATTACK_COLOR: Color = Color::srgb(0., 0., 0.);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerAttack {
    timer: Timer,
}

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

pub fn player_attack(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = player_query.single();
    
    if keyboard_input.pressed(KeyCode::KeyX) {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        player_transform.translation.x,
                        player_transform.translation.y,
                        -1.
                    ),
                    scale: Vec3::new(40., 40., 1.),
                    ..default()
                },
                sprite: Sprite {
                    color: PLAYER_ATTACK_COLOR,
                    ..default()
                },
                ..default()
            },
            PlayerAttack {
                timer: Timer::new(Duration::from_millis(100), TimerMode::Once),
            }
        ));
    }
}

pub fn player_attack_check_for_enemy_collisions(
    mut commands: Commands,
    player_attack_query: Query<(Entity, &Transform), With<PlayerAttack>>,
    enemies_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (player_attack_entity, player_attack_transform) in &player_attack_query {
        let player_attack_bounding_box = Aabb2d::new(
            player_attack_transform.translation.truncate(),
            player_attack_transform.scale.truncate() / 2.,
        );

        for (enemy_entity, enemy_transform) in &enemies_query {
            let enemy_bounding_box = Aabb2d::new(
                enemy_transform.translation.truncate(),
                enemy_transform.scale.truncate() / 2.,
            );

            if player_attack_bounding_box.intersects(&enemy_bounding_box) {
                commands.entity(enemy_entity).despawn();
                commands.entity(player_attack_entity).despawn()
            }
        }
    }
}

pub fn remove_player_attacks(
    mut commands: Commands,
    mut player_attack_query: Query<(Entity, &mut PlayerAttack)>,
    time: Res<Time>,
) {
    for (player_attack_entity, mut player_attack) in player_attack_query.iter_mut() {
        player_attack.timer.tick(time.delta());

        if player_attack.timer.finished() {
            commands.entity(player_attack_entity).despawn();
        }
    }
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


