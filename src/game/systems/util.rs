use bevy::asset::AssetServer;
use bevy::audio::{AudioPlayer, AudioSource, PlaybackSettings};
use bevy::prelude::{Commands, Res};

pub fn add_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::<AudioSource>::new(asset_server.load("sounds/Original Tetris Sound.ogg")),
        PlaybackSettings::LOOP,
    ));
}
