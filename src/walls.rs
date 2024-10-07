use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::collider::{Collider, Collision};

const WALL_COLOR: Color = Color::srgb(120.0, 120.0, 120.0);
const WALL_THICKNESS: f32 = 5.;

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    wall: Wall,
}

#[derive(Component)]
pub struct Wall;

impl WallBundle {
    fn new(
        location_start_x: f32,
        location_start_y: f32,
        size_x: f32,
        size_y: f32,
    ) -> WallBundle {
        
        /*
         * Adjust start points since scaling starts at the centers
         * Draws rectangle by starting in the bottom left corner,
         * then drawing right and up.
         */
        let start_x: f32 = location_start_x + size_x / 2.;
        let start_y: f32 = location_start_y + size_y / 2.;

        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec2::new(start_x, start_y).extend(0.0),
                    scale: Vec2::new(size_x, size_y).extend(0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            wall: Wall,
            collider: Collider,
        }
    }
}

pub fn wall_collision(player_bounding_box: &Aabb2d, wall: Aabb2d) -> Option<Collision> {
    if !player_bounding_box.intersects(&wall) {
        return None;
    }

    let closest = wall.closest_point(player_bounding_box.bounding_circle().center);
    let offset = player_bounding_box.bounding_circle().center - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}

pub fn spawn_walls(commands: &mut Commands) {
    // WALL_THICKNESS is the third argument for vertical lines,
    // fourth argument for horizontal lines
    commands.spawn(WallBundle::new(0., 65., 200., WALL_THICKNESS));
    commands.spawn(WallBundle::new(0., -5., 130., WALL_THICKNESS));
    commands.spawn(WallBundle::new(125., -305., WALL_THICKNESS, 300.));
    commands.spawn(WallBundle::new(125., -305., 370.0, WALL_THICKNESS));
    commands.spawn(WallBundle::new(195., -235., WALL_THICKNESS, 300.));
    commands.spawn(WallBundle::new(195., -235., 300.0, WALL_THICKNESS));
}

