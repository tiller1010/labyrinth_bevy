use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

const ENEMY_SIZE: Vec2 = Vec2::new(10., 10.,);
const ENEMY_COLOR: Color = Color::srgb(50., 0., 50.);
const INITIAL_ENEMY_DIRECTION: Vec2 = Vec2::new(1., 1.);
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
// pub fn apply_enemy_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

pub fn spawn_enemy(commands: &mut Commands) {
    commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(80., -80., 0.),
                    scale: ENEMY_SIZE.extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: ENEMY_COLOR,
                    ..default()
                },
                ..default()
            },
            Enemy,
            // Velocity(INITIAL_ENEMY_DIRECTION.normalize() * ENEMY_SPEED),
            Velocity(Vec2::new(50., 10.)),
    ));
}

pub fn update_enemy_movement(
    mut enemy_query: Query<&mut Velocity, With<Enemy>>,
) {
    let mut enemy_velocity = enemy_query.single_mut();

    let random_direction: Direction = rand::random();

    if random_direction == Direction::Left {
        enemy_velocity.x = -10.;
        enemy_velocity.y = 0.;
    } else if random_direction == Direction::Right {
        enemy_velocity.x = 10.;
        enemy_velocity.y = 0.;
    } else if random_direction == Direction::Up {
        enemy_velocity.x = 0.;
        enemy_velocity.y = 10.;
    } else {
        enemy_velocity.x = 0.;
        enemy_velocity.y = -10.;
    }


    // enemy_velocity = match random_direction {
    //     Left => { ..enemy.velocity, x: -10. },
    //     Right => { ..enemy.velocity, x: -10. },
    //     Up => { ..enemy.velocity, y: -10. },
    //     _ => { ..enemy.velocity, y: -10. },
    // }

}

