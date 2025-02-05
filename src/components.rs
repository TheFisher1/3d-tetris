use bevy::prelude::*;

#[derive(Component)]
pub struct Tetromino;

#[derive(Component)]
pub struct GridWall;

#[derive(Component)]
pub struct Falling {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Stopped;

#[derive(Component)]
pub struct MovementCooldown {
    pub timer: Timer,
}

#[derive(Component)]
pub struct GameCamera {
    pub angle: f32,
    pub height: f32,
} 