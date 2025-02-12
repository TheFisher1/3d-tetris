use bevy::prelude::*;

#[derive(Component)]
pub struct Tetromino {
    pub id: isize,
}

#[derive(Component)]
pub struct TetrominoBlock {
    pub id: isize,
}

#[derive(Component)]
pub struct GridWall;

#[derive(Component)]
pub struct Falling {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Active;

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

#[derive(Component)]
pub enum RotateButton {
    Left,
    Right,
    Down,
    Up,
}
