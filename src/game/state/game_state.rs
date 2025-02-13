use crate::game_elements::Tetromino;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GameState {
    curr_tetromino: Tetromino,
    next_tetromino: Tetromino,
    level: f32,
    xp: f32,
}
