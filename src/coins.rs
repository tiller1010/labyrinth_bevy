use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::player::player::Player;

#[derive(Component)]
pub struct Coin;

#[derive(Resource, Deref, DerefMut)]
pub struct Score(pub usize);

#[derive(Component)]
pub struct ScoreboardUi;

const COIN_SIZE: Vec2 = Vec2::new(10., 10.);
const COIN_COLOR: Color = Color::srgb(120., 120., 0.);

pub fn spawn_coins(
    commands: &mut Commands,
) {
    spawn_coin(commands, 80., 80.);
    spawn_coin(commands, 120., 120.);
    spawn_coin(commands, 250., 340.);
    spawn_coin(commands, 440., 440.);
    spawn_coin(commands, 80., 440.);
}

fn spawn_coin(
    commands: &mut Commands,
    x_position: f32,
    y_position: f32,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(x_position, y_position, 0.),
                scale: COIN_SIZE.extend(1.),
                ..default()
            },
            sprite: Sprite {
                color: COIN_COLOR,
                ..default()
            },
            ..default()
        },
        Coin,
    ));
}

pub fn check_for_player_collisions_with_coins(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut player_query: Query<&Transform, With<Player>>,
    coins_query: Query<(Entity, &Transform), With<Coin>>,
) {
    let player_transform = player_query.single_mut();
    let player_bounding_box = Aabb2d::new(
        player_transform.translation.truncate(),
        player_transform.scale.truncate() / 2.,
    );

    for (coin_entity, coin_transform) in &coins_query {
        let coin_bounding_box = Aabb2d::new(
            coin_transform.translation.truncate(),
            coin_transform.scale.truncate() / 2.,
        );

        if coin_bounding_box.intersects(&player_bounding_box) {
            **score += 1;
            commands.entity(coin_entity).despawn();
        }
    }
}

pub fn spawn_scoreboard(
    commands: &mut Commands,
) {
   commands.spawn((
        ScoreboardUi,
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: 20.,
                    color: Color::srgb(80., 80., 80.),
                    ..default()
                }
            ),
            TextSection::from_style(
                TextStyle {
                    font_size: 20.,
                    color: Color::srgb(80., 80., 80.),
                    ..default()
                }
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.),
            left: Val::Px(5.),
            ..default()
        }),
   ));
}

pub fn update_scoreboard(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreboardUi>>,
) {
    let mut text = query.single_mut();
    text.sections[1].value = score.to_string();
}

