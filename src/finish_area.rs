use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::player::Player;
use crate::collider::Collider;

const FINISH_AREA_COLOR: Color = Color::srgb(0., 115., 0.);

#[derive(Component)]
struct WinningMessage;

#[derive(Component)]
pub struct FinishArea;

#[derive(Bundle)]
pub struct FinishAreaBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    finish_area: FinishArea,
}

impl FinishAreaBundle {
    fn new(
        location_start_x: f32,
        location_start_y: f32,
        size_x: f32,
        size_y: f32,
    ) -> FinishAreaBundle {
        
        /*
         * Adjust start points since scaling starts at the centers
         * Draws rectangle by starting in the bottom left corner,
         * then drawing right and up.
         */
        let start_x: f32 = location_start_x + size_x / 2.;
        let start_y: f32 = location_start_y + size_y / 2.;

        FinishAreaBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec2::new(start_x, start_y).extend(0.0),
                    scale: Vec2::new(size_x, size_y).extend(0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: FINISH_AREA_COLOR,
                    ..default()
                },
                ..default()
            },
            finish_area: FinishArea,
            collider: Collider,
        }
    }
}

pub fn finish_area_collision(
    commands: &mut Commands,
    player_bounding_box: &Aabb2d,
    finish_area_collider_query: &Query<&Transform, (With<FinishArea>, Without<Player>)>
) {
    for finish_area_transform in finish_area_collider_query {
        let finish_area_bounding_box = Aabb2d::new(
            finish_area_transform.translation.truncate(),
            finish_area_transform.scale.truncate() / 2.,
        );
        if player_bounding_box.intersects(&finish_area_bounding_box) {
            spawn_winning_message(commands);
        }
    }
}

pub fn spawn_finish_area(commands: &mut Commands) {
    commands.spawn(FinishAreaBundle::new(575., 580., 25., 25.));
}

fn spawn_winning_message(commands: &mut Commands) {
    commands.spawn((
        WinningMessage,
        TextBundle::from_sections([
            TextSection::new(
                "You Win!",
                TextStyle {
                    font_size: 40.,
                    color: Color::srgb(0., 120., 0.),
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            left: Val::Px(5.),
            ..default()
        }),
    ));
}

