use crate::game::game_elements::Tetromino;
use bevy::prelude::{Resource, States};

#[derive(States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameStatus {
    StartPage,
    Info,
    Playing,
    GameOver,
}

#[derive(Resource)]
pub struct GameState {
    pub curr_tetromino: Tetromino,
    pub next_tetromino: Tetromino,
    pub level: u32,
    pub points: f32,
}

impl GameState {
    pub fn new(level: u32, points: f32) -> Self {
        GameState {
            next_tetromino: Tetromino { id: 0 },
            curr_tetromino: Tetromino { id: 1 },
            level,
            points,
        }
    }

    pub fn reset(&mut self) {
        self.curr_tetromino = Tetromino { id: 0 };
        self.next_tetromino = Tetromino { id: 1 };
        self.level = 0;
        self.points = 0.0;
    }
}
