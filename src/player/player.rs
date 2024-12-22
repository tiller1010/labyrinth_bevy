use bevy::{
    math::bounding::Aabb2d,
    prelude::*,
};
use std::time::Duration;

use crate::collider::{Collider};
use crate::finish_area::{FinishArea, finish_area_collision};

const PLAYER_SIZE: Vec2 = Vec2::new(10.0, 10.0);
pub const PLAYER_SPEED: f32 = 200.;

#[derive(PartialEq)]
pub enum PlayerFacingDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
pub struct Player {
    pub alive: bool,
    pub player_attack_cooldown_timer: Timer,
    pub player_facing_direction: PlayerFacingDirection,
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

pub fn trigger_player_walking_animation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut AnimationConfig, &mut TextureAtlas)>,
) {
    let (player, mut animation, mut atlas) = player_query.single_mut();

    if !player.alive { return };

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

pub fn move_player(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Player, &mut Transform, &mut Sprite), With<Player>>,
    finish_area_collider_query: Query<&Transform, (With<FinishArea>, Without<Player>)>,
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

    let player_bounding_box = Aabb2d::new(
        player_transform.translation.truncate(), 
        player_transform.scale.truncate() / 2.,
    );

    // Start finish area collision detection
    finish_area_collision(&mut commands, &player_bounding_box, &finish_area_collider_query);
}

