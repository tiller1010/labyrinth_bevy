use bevy::{
    input::common_conditions::input_pressed,
    prelude::*,
};

mod player;
mod walls;
mod collider;
mod camera;
mod finish_area;
mod maze;
mod enemy;
mod coins;
mod music;
mod menu;
mod game_state;

use crate::player::player::{
    spawn_player,
    move_player,
    execute_player_walking_animations,
    trigger_player_walking_animation,
};
use crate::player::player_attack::{
    player_attack,
    player_attack_check_for_enemy_collisions,
    remove_player_attacks,
    cooldown_player_attack_timer,
};
use crate::walls::{
    spawn_walls,
    player_wall_collistions,
};
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
use crate::music::play_music;
use crate::menu::{
    MenuState,
    OnMainMenuScreen,
    menu_setup,
    menu_action,
    main_menu_setup,
};
use crate::game_state::{
    GameState,
};

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

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
        .with_text_justify(JustifyText::Right)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.),
            right: Val::Px(5.),
            ..default()
        }),
    ));
}

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    explain_game(&mut commands);
    spawn_walls(&mut commands);
    spawn_player(&mut commands, &asset_server, &mut texture_atlas_layouts);
    spawn_finish_area(&mut commands);
    spawn_enemies(&mut commands, &asset_server);
    spawn_coins(&mut commands);
    spawn_scoreboard(&mut commands);
    play_music(&mut commands, &asset_server);
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<MenuState>()
        .init_state::<GameState>()
        .add_event::<CollisionEvent>()
        .insert_resource(Score(0))
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_camera.run_if(in_state(GameState::Game)))
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        .add_systems(OnExit(GameState::Menu), despawn_screen::<OnMainMenuScreen>)
        .add_systems(OnEnter(GameState::Game), game_setup)
        .add_systems(
            Update,
            execute_player_walking_animations
            .run_if(in_state(GameState::Game))
        )
        .add_systems(
            Update,
            (
                trigger_player_walking_animation.run_if(input_pressed(KeyCode::ArrowLeft)),
                trigger_player_walking_animation.run_if(input_pressed(KeyCode::ArrowRight)),
                trigger_player_walking_animation.run_if(input_pressed(KeyCode::ArrowUp)),
                trigger_player_walking_animation.run_if(input_pressed(KeyCode::ArrowDown)),
            )
            .run_if(in_state(GameState::Game))
        )
        .add_systems(
            FixedUpdate,
            (
                apply_enemy_velocity,
                move_player,
                player_wall_collistions,
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
            .chain()
            .run_if(in_state(GameState::Game))
        )
        .add_systems(
            Update,
            menu_action.run_if(in_state(GameState::Menu))
        )
        .run();
}

