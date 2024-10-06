use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

mod player;
mod walls;
mod collider;

use crate::player::{Player, spawn_player, move_player};
use crate::walls::spawn_walls;
use crate::collider::{Collider, Collision, CollisionEvent};

const FINISH_AREA_COLOR: Color = Color::srgb(0., 120., 0.);

const CAM_LERP_FACTOR: f32 = 2.;

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

fn setup_camera(mut commands: Commands) {
    commands.spawn((
            Camera2dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                ..default()
            },
    ));
}

fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let Ok(player) = player.get_single() else {
        return;
    };

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    camera.translation = camera
        .translation
        .lerp(direction, time.delta_seconds() * CAM_LERP_FACTOR);
}

fn setup(
    mut commands: Commands,
) {
    // Winning message spawn
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

    spawn_walls(&mut commands);
    spawn_player(&mut commands);
}

fn finish_area_collision(player: Aabb2d, finish: Aabb2d) {
     !player.intersects(&finish); 
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<CollisionEvent>()
        .add_systems(Startup, (setup, setup_camera))
        .add_systems(
            FixedUpdate,
            (
                move_player,
                update_camera,
            )
            .chain(),
        )
        .run();

}

