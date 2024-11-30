use bevy:: prelude::*;

mod player;
mod walls;
mod collider;
mod camera;
mod finish_area;
mod maze;
mod enemy;
mod coins;

use crate::player::{
    spawn_player,
    move_player,
    player_attack,
    player_attack_check_for_enemy_collisions,
    remove_player_attacks,
    cooldown_player_attack_timer,
};
use crate::walls::spawn_walls;
use crate::collider::CollisionEvent;
use crate::camera::{setup_camera, update_camera};
use crate::finish_area::spawn_finish_area;
use crate::enemy::{
    spawn_enemies,
    apply_enemy_velocity,
    update_enemy_movement,
    check_for_player_collisions_with_enemies,
};
use crate::coins::{
    Score,
    spawn_coins,
    spawn_scoreboard,
    update_scoreboard,
    check_for_player_collisions_with_coins,
};

fn explain_game(
    commands: &mut Commands,
) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Use arrow keys to move, press x to attack.\nAvoid enemies and try to find the green finish area.",
                TextStyle {
                    font_size: 20.,
                    color: Color::srgb(80., 80., 80.),
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.),
            right: Val::Px(5.),
            ..default()
        }),
    ));
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    explain_game(&mut commands);
    spawn_walls(&mut commands);
    spawn_player(&mut commands, &asset_server);
    spawn_finish_area(&mut commands);
    spawn_enemies(&mut commands, &asset_server);
    spawn_coins(&mut commands);
    spawn_scoreboard(&mut commands);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<CollisionEvent>()
        .insert_resource(Score(0))
        .add_systems(Startup, (setup, setup_camera))
        .add_systems(
            FixedUpdate,
            (
                apply_enemy_velocity,
                move_player,
                update_camera,
                update_enemy_movement,
                check_for_player_collisions_with_enemies,
                player_attack,
                player_attack_check_for_enemy_collisions,
                remove_player_attacks,
                cooldown_player_attack_timer,
                update_scoreboard,
                check_for_player_collisions_with_coins,
            )
            .chain(),
        )
        .run();

}

