use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::collider::{Collider, Collision, CollisionEvent};
use crate::maze::draw_maze;
use crate::player::player::{
    Player,
    PLAYER_SPEED,
};

const WALL_COLOR: Color = Color::srgb(120.0, 120.0, 120.0);
const WALL_THICKNESS: f32 = 5.;

#[derive(Bundle)]
pub struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    wall: Wall,
}

#[derive(Component)]
pub struct Wall;

impl WallBundle {
    pub fn new(
        location_start_x: f32,
        location_start_y: f32,
        location_end_x: f32,
        location_end_y: f32,
    ) -> WallBundle {

        let size_x: f32 = if location_start_x == location_end_x {
            WALL_THICKNESS
        } else {
            (location_end_x - location_start_x).abs()
        };

        let size_y: f32 = if location_start_y == location_end_y {
            WALL_THICKNESS
        } else {
            (location_end_y - location_start_y).abs()
        };
        
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

pub fn player_wall_collistions(
    mut player_query: Query<&mut Transform, With<Player>>,
    wall_collider_query: Query<&Transform, (With<Wall>, Without<Player>)>,
    time: Res<Time>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let mut player_transform = player_query.single_mut();

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
}

pub fn spawn_walls(commands: &mut Commands) {
    draw_maze(commands);
}

