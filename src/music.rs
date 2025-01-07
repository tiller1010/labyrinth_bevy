use bevy::prelude::*;
use bevy::audio::{PlaybackMode, Volume};

pub fn play_music(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    commands.spawn(
        AudioBundle {
            source: asset_server.load::<AudioSource>("game_song.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new(1.),
                ..default()
            },
        }
    );
}

