use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use std::time::Duration;

use crate::enemy::Enemy;
use crate::player::player::{
    Player,
    PlayerFacingDirection,
};

#[derive(Component)]
pub struct PlayerAttack {
    active_timer: Timer,
}

pub fn player_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Player, &Transform), With<Player>>,
) {
    let (mut player, player_transform) = player_query.single_mut();
    
    if !player.alive { return };
    
    if keyboard_input.pressed(KeyCode::KeyX) && player.player_attack_cooldown_timer.finished() {
        player.player_attack_cooldown_timer = Timer::new(Duration::from_millis(500), TimerMode::Once);

        let attack_location = match player.player_facing_direction {
            PlayerFacingDirection::Left => Vec2::new(
                player_transform.translation.x - 30.,
                player_transform.translation.y
            ),
            PlayerFacingDirection::Right => Vec2::new(
                player_transform.translation.x + 30.,
                player_transform.translation.y
            ),
            PlayerFacingDirection::Up => Vec2::new(
                player_transform.translation.x,
                player_transform.translation.y + 30.
            ),
            PlayerFacingDirection::Down => Vec2::new(
                player_transform.translation.x,
                player_transform.translation.y - 30.
            ),
        };

        // Rotation in radians
        let attack_rotation = match player.player_facing_direction {
            PlayerFacingDirection::Left => Quat::from_rotation_z(-1.57),
            PlayerFacingDirection::Right => Quat::from_rotation_z(1.57),
            PlayerFacingDirection::Up => Quat::from_rotation_z(3.14),
            PlayerFacingDirection::Down => Quat::from_rotation_z(0.),
        };

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sword.png"),
                transform: Transform {
                    translation: attack_location.extend(-1.),
                    scale: Vec3::new(20., 40., 1.),
                    rotation: attack_rotation,
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..default()
                },
                ..default()
            },
            PlayerAttack {
                active_timer: Timer::new(Duration::from_millis(100), TimerMode::Once),
            }
        ));
    }
}

pub fn player_attack_check_for_enemy_collisions(
    mut commands: Commands,
    player_attack_query: Query<&Transform, With<PlayerAttack>>,
    enemies_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for player_attack_transform in &player_attack_query {
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
        player_attack.active_timer.tick(time.delta());

        if player_attack.active_timer.finished() {
            commands.entity(player_attack_entity).despawn();
        }
    }
}

pub fn cooldown_player_attack_timer(
    mut player_query: Query<&mut Player>,
    time: Res<Time>,
) {
    for mut player in player_query.iter_mut() {
        player.player_attack_cooldown_timer.tick(time.delta());
    }
}

