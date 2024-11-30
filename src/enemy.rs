use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::collider::Collision;
use crate::walls::{Wall, wall_collision};
use crate::player::Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

const ENEMY_SIZE: Vec2 = Vec2::new(10., 10.,);
const INITIAL_ENEMY_DIRECTION: Vec2 = Vec2::new(1., 0.);
const ENEMY_SPEED: f32 = 50.;

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::Left,
            1 => Direction::Right,
            3 => Direction::Up,
            _ => Direction::Down,
        }
    }
}

pub fn apply_enemy_velocity(mut query: Query<(&mut Transform, &Velocity), With<Enemy>>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    x_position: f32,
    y_position: f32
) {
    commands.spawn((
            SpriteBundle {
                texture: asset_server.load("enemy.png"),
                transform: Transform {
                    translation: Vec3::new(x_position, y_position, 0.),
                    scale: ENEMY_SIZE.extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(2., 2.)),
                    ..default()
                },
                ..default()
            },
            Enemy,
            Velocity(Vec2::new(INITIAL_ENEMY_DIRECTION.x * ENEMY_SPEED, INITIAL_ENEMY_DIRECTION.y * ENEMY_SPEED)),
    ));
}

pub fn spawn_enemies(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    spawn_enemy(commands, asset_server, 80., 80.);
    spawn_enemy(commands, asset_server, 120., 120.);
    spawn_enemy(commands, asset_server, 250., 340.);
    spawn_enemy(commands, asset_server, 440., 440.);
    spawn_enemy(commands, asset_server, 80., 440.);
}

pub fn update_enemy_movement(
    mut enemy_query: Query<(&mut Transform, &mut Velocity), With<Enemy>>,
    wall_collider_query: Query<&Transform, (With<Wall>, Without<Enemy>)>,
) {
    for (mut enemy_transform, mut enemy_velocity) in &mut enemy_query {

        let enemy_bounding_box = Aabb2d::new(
            enemy_transform.translation.truncate(), 
            enemy_transform.scale.truncate() / 2.,
        );

        for wall_transform in &wall_collider_query {
            let collision = wall_collision(
                &enemy_bounding_box,
                Aabb2d::new(
                    wall_transform.translation.truncate(),
                    wall_transform.scale.truncate() / 2.,
                ),
            );

            if let Some(collision) = collision {
                let mut reflect_x = 0.;
                let mut reflect_y = 0.;

                match collision {
                    Collision::Left => reflect_x = -7.9,
                    Collision::Right => reflect_x = 7.9,
                    Collision::Top => reflect_y = 7.9,
                    Collision::Bottom => reflect_y = -7.9,
                }

                if reflect_x != 0. {
                    enemy_transform.translation.x = enemy_transform.translation.x + reflect_x;
                }

                if reflect_y != 0. {
                    enemy_transform.translation.y = enemy_transform.translation.y + reflect_y;
                }

                // Set velocity by random direction, but discourage going in the same direction & backtracking based collision
                let random_direction: Direction = rand::random();
                if random_direction == Direction::Left
                    && collision != Collision::Left
                    && collision != Collision::Right
                {
                    enemy_velocity.x = -1. * ENEMY_SPEED;
                    enemy_velocity.y = 0.;
                    return;
                } else if random_direction == Direction::Right
                    && collision != Collision::Left
                    && collision != Collision::Right
                {
                    enemy_velocity.x = 1. * ENEMY_SPEED;
                    enemy_velocity.y = 0.;
                    return;
                } else if random_direction == Direction::Up
                    && collision != Collision::Top
                    && collision != Collision::Bottom
                {
                    enemy_velocity.x = 0.;
                    enemy_velocity.y = 1. * ENEMY_SPEED;
                    return;
                } else if random_direction == Direction::Down
                    && collision != Collision::Top
                    && collision != Collision::Bottom
                {
                    enemy_velocity.x = 0.;
                    enemy_velocity.y = -1. * ENEMY_SPEED;
                    return;
                }

                // Set velocity by random direction, but discourage backtracking based collision
                let random_direction: Direction = rand::random();
                if random_direction == Direction::Left && collision != Collision::Left {
                    enemy_velocity.x = -1. * ENEMY_SPEED;
                    enemy_velocity.y = 0.;
                    return;
                } else if random_direction == Direction::Right && collision != Collision::Right {
                    enemy_velocity.x = 1. * ENEMY_SPEED;
                    enemy_velocity.y = 0.;
                    return;
                } else if random_direction == Direction::Up && collision != Collision::Top {
                    enemy_velocity.x = 0.;
                    enemy_velocity.y = 1. * ENEMY_SPEED;
                    return;
                } else if random_direction == Direction::Down && collision != Collision::Bottom {
                    enemy_velocity.x = 0.;
                    enemy_velocity.y = -1. * ENEMY_SPEED;
                    return;
                }

                // Fallback to allow for same direction or backtracking
                let random_direction: Direction = rand::random();
                if random_direction == Direction::Left {
                    enemy_velocity.x = -1. * ENEMY_SPEED;
                    enemy_velocity.y = 0.;
                    return;
                } else if random_direction == Direction::Right {
                    enemy_velocity.x = 1. * ENEMY_SPEED;
                    enemy_velocity.y = 0.;
                    return;
                } else if random_direction == Direction::Up {
                    enemy_velocity.x = 0.;
                    enemy_velocity.y = 1. * ENEMY_SPEED;
                    return;
                } else {
                    enemy_velocity.x = 0.;
                    enemy_velocity.y = -1. * ENEMY_SPEED;
                    return;
                }

            }
        }
    }
}

pub fn check_for_player_collisions(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &Transform), With<Player>>,
    enemies_query: Query<&Transform, With<Enemy>>,
) {
    let (mut player, player_transform) = player_query.single_mut();
    let player_bounding_box = Aabb2d::new(
        player_transform.translation.truncate(),
        player_transform.scale.truncate() / 2.,
    );

    for enemy_transform in &enemies_query {
        let enemy_bounding_box = Aabb2d::new(
            enemy_transform.translation.truncate(),
            enemy_transform.scale.truncate() / 2.,
        );

        if enemy_bounding_box.intersects(&player_bounding_box) {
            player.alive = false;

            commands.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "Game Over",
                        TextStyle {
                            font_size: 40.,
                            color: Color::srgb(120., 0., 0.),
                            ..default()
                        },
                    ),
                ])
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(5.),
                    right: Val::Px(5.),
                    ..default()
                }),
            ));
        }
    }

}

