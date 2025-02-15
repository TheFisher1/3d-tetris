use bevy::asset::AssetServer;
use bevy::audio::{AudioPlayer, AudioSource, PlaybackSettings};
use bevy::prelude::*;

pub fn add_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::<AudioSource>::new(asset_server.load("sounds/Original Tetris Sound.ogg")),
        PlaybackSettings::LOOP,
    ));
}

pub fn add_image(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_image = asset_server.load("images/tetris-logo.png");

    commands.spawn((
        Sprite::from_image(background_image),
        Transform::from_xyz(0.0, 200.0, 0.0)
                .with_scale(Vec3::new(2.0, 1.5, 1.0))
    ));
}
