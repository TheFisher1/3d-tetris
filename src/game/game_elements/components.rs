use crate::game::game_elements::TYPES;
use bevy::prelude::*;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum TetrominoType {
    I,
    L,
    S,
    O,
}

impl TetrominoType {
    fn random() -> TetrominoType {
        let mut rng = rand::rng();
        let idx: i32 = rng.random_range(0..TYPES.len() as i32);

        TYPES[idx as usize].clone()
    }
}

#[derive(Component, Clone, Debug)]
pub struct Tetromino {
    pub id: isize,
    pub tetromino_type: TetrominoType,
}

impl Iterator for Tetromino {
    type Item = Tetromino;

    fn next(&mut self) -> Option<Tetromino> {
        Some(Tetromino {
            id: self.id + 1,
            tetromino_type: TetrominoType::random(),
        })
    }
}

impl Default for Tetromino {
    fn default() -> Self {
        Tetromino {
            id: 0,
            tetromino_type: TetrominoType::random(),
        }
    }
}

#[derive(Component, Debug)]
pub struct TetrominoBlock {
    pub id: isize,
}

#[derive(Component)]
pub struct Falling {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Active;

#[derive(Component)]
pub struct Stopped;

#[derive(Component)]
pub struct JustSpawned;

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
