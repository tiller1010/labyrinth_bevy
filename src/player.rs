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
const PLAYER_SPEED: f32 = 200.;

#[derive(PartialEq)]
enum PlayerFacingDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
pub struct Player {
    pub alive: bool,
    player_attack_cooldown_timer: Timer,
    player_facing_direction: PlayerFacingDirection,
}

#[derive(Component)]
pub struct PlayerAttack {
    active_timer: Timer,
}

pub fn spawn_player(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("player-texture-atlas.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(12, 15), 6, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_config = AnimationConfig::new(0, 0, 10);

    // Player spawn
    commands.spawn((
        SpriteBundle {
            texture: texture.clone(),
            transform: Transform {
                translation: Vec3::new(20., 10., 0.),
                scale: PLAYER_SIZE.extend(1.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(2., 2.)),
                ..default()
            },
            ..default()
        },
        Player {
            alive: true,
            player_attack_cooldown_timer: Timer::new(Duration::from_millis(0), TimerMode::Once),
            player_facing_direction: PlayerFacingDirection::Down,
        },
        Collider,
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: animation_config.first_sprite_index,
        },
        animation_config,
    ));
}

#[derive(Component)]
pub struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1. / (fps as f32)), TimerMode::Once)
    }
}

pub fn trigger_animation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut AnimationConfig, &mut TextureAtlas)>,
) {
    let (mut animation, mut atlas) = query.single_mut();
    if atlas.index == 0 || atlas.index == 3 {
        if keyboard_input.pressed(KeyCode::ArrowLeft)
        || keyboard_input.pressed(KeyCode::ArrowRight)
        || keyboard_input.pressed(KeyCode::ArrowDown)
        {
            atlas.index = 1;
            animation.first_sprite_index = 1;
            animation.last_sprite_index = 2;
        } else if keyboard_input.pressed(KeyCode::ArrowUp) {
            atlas.index = 4;
            animation.first_sprite_index = 4;
            animation.last_sprite_index = 5;
        }

        animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
    }
}

pub fn execute_player_walking_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut TextureAtlas)>,
) {
    for (mut animation, mut atlas) in &mut query {
        animation.frame_timer.tick(time.delta());

        if animation.frame_timer.just_finished() {
            if atlas.index == animation.last_sprite_index {
                if animation.first_sprite_index == 0 {
                    atlas.index = 0;
                } else {
                    atlas.index = animation.first_sprite_index - 1;
                }
            } else {
                atlas.index += 1;
                animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
            }
        }
    }
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

pub fn move_player(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform, &mut Sprite), With<Player>>,
    time: Res<Time>,
    wall_collider_query: Query<&Transform, (With<Wall>, Without<Player>)>,
    finish_area_collider_query: Query<&Transform, (With<FinishArea>, Without<Player>)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut player, mut player_transform, mut player_sprite) = player_query.single_mut();

    if !player.alive { return };

    let mut direction_x = 0.0;
    let mut direction_y = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction_x -= 1.0;
        player.player_facing_direction = PlayerFacingDirection::Left;
        player_sprite.flip_x = true;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction_x += 1.0;
        player.player_facing_direction = PlayerFacingDirection::Right;
        player_sprite.flip_x = false;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction_y += 1.0;
        player.player_facing_direction = PlayerFacingDirection::Up;
        player_sprite.flip_x = false;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction_y -= 1.0;
        player.player_facing_direction = PlayerFacingDirection::Down;
        player_sprite.flip_x = false;
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
                Collision::Left => reflect_x = -1.,
                Collision::Right => reflect_x = 1.,
                Collision::Top => reflect_y = 1.,
                Collision::Bottom => reflect_y = -1.,
            }

            if reflect_x != 0. {
                player_transform.translation.x = player_transform.translation.x + reflect_x * PLAYER_SPEED * time.delta_seconds();
            }

            if reflect_y != 0. {
                player_transform.translation.y = player_transform.translation.y + reflect_y * PLAYER_SPEED * time.delta_seconds();
            }
        }
    }
    // End Wall Collision Detection

    // Start finish area collision detection
    finish_area_collision(&mut commands, &player_bounding_box, &finish_area_collider_query);
}


