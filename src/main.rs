use bevy:: prelude::*;

mod player;
mod walls;
mod collider;
mod camera;
mod finish_area;
mod maze;

use crate::player::{spawn_player, move_player};
use crate::walls::spawn_walls;
use crate::collider::CollisionEvent;
use crate::camera::{setup_camera, update_camera};
use crate::finish_area::spawn_finish_area;

fn setup(
    mut commands: Commands,
) {
    spawn_walls(&mut commands);
    spawn_player(&mut commands);
    spawn_finish_area(&mut commands);
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

