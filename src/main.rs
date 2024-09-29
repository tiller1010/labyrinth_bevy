use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 50.0);
const PLAYER_COLOR: Color = Color::srgb(50.0, 0.0, 0.0);
const PLAYER_SPEED: f32 = 250.;

const WALL_COLOR: Color = Color::srgb(120.0, 120.0, 120.0);
const WALL_THICKNESS: f32 = 5.;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

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
            collider: Collider,
        }
    }
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(WallBundle::new(0., 65., 200., WALL_THICKNESS));
    commands.spawn(WallBundle::new(0., -5., 130., WALL_THICKNESS));
    commands.spawn(WallBundle::new(125., -235., WALL_THICKNESS, 230.));
    commands.spawn(WallBundle::new(195., -235., WALL_THICKNESS, 300.));

    commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: PLAYER_SIZE.extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: PLAYER_COLOR,
                    ..default()
                },
                ..default()
            },
            Player,
            Collider,
    ));
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    collider_query: Query<&Transform, (With<Collider>, Without<Player>)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let mut player_transform = query.single_mut();
    let mut direction_x = 0.0;
    let mut direction_y = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction_x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction_x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction_y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction_y -= 1.0;
    }

    let new_player_position_x = player_transform.translation.x + direction_x * PLAYER_SPEED * time.delta_seconds();
    let new_player_position_y = player_transform.translation.y + direction_y * PLAYER_SPEED * time.delta_seconds();

    player_transform.translation.x = new_player_position_x;
    player_transform.translation.y = new_player_position_y;
    // End move player

    // Start Wall Collision Detection
    for wall_transform in &collider_query {
        let collision = wall_collision(
            Aabb2d::new(
                player_transform.translation.truncate(), 
                player_transform.scale.truncate() / 2.,
            ),
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
                Collision::Left => reflect_x = -7.9,
                Collision::Right => reflect_x = 7.9,
                Collision::Top => reflect_y = 7.9,
                Collision::Bottom => reflect_y = -7.9,
            }

            if reflect_x != 0. {
                player_transform.translation.x = player_transform.translation.x + reflect_x;
            }

            if reflect_y != 0. {
                player_transform.translation.y = player_transform.translation.y + reflect_y;
            }
        }
    }
    // End Wall Collision Detection

}

// fn check_for_collisions(
//     mut player_query: Query<&mut Transform, With<Player>>,
//     collider_query: Query<&Transform, With<Collider>>,
//     mut collision_events: EventWriter<CollisionEvent>,
// ) {
// ...
// }


fn wall_collision(player: Aabb2d, wall: Aabb2d) -> Option<Collision> {
    if !player.intersects(&wall) {
        return None;
    }

    let closest = wall.closest_point(player.bounding_circle().center);
    let offset = player.bounding_circle().center - closest;
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                move_player,
                // check_for_collisions,
            )
            .chain(),
        )
        .run();

}

