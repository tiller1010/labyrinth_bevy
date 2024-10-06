use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use crate::collider::Collider;

const FINISH_AREA_COLOR: Color = Color::srgb(0., 120., 0.);

#[derive(Component)]
struct WinningMessage;

#[derive(Component)]
struct FinishArea {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl FinishArea {
    fn new(
        location_start_x: f32,
        location_start_y: f32,
        size_x: f32,
        size_y: f32,
    ) -> FinishArea {
        
        /*
         * Adjust start points since scaling starts at the centers
         * Draws rectangle by starting in the bottom left corner,
         * then drawing right and up.
         */
        let start_x: f32 = location_start_x + size_x / 2.;
        let start_y: f32 = location_start_y + size_y / 2.;

        FinishArea {
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
            collider: Collider,
        }
    }
}

fn finish_area_collision(player: Aabb2d, finish: Aabb2d) {
     !player.intersects(&finish); 
}

pub fn spawn_finish_area(commands: &mut Commands) {
}

pub fn spawn_winning_message(commands: &mut Commands) {
    commands.spawn((
            WinningMessage,
            TextBundle::from_sections([
                TextSection::new(
                    "You Win!",
                    TextStyle {
                        font_size: 40.,
                        color: Color::srgb(0., 0., 0.),
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

