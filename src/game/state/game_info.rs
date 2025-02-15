use crate::game::game_elements::Tetromino;
use crate::game::systems::RowCleaned;
use bevy::math::ops::exp;
use bevy::prelude::{EventReader, ResMut, Resource, States};

#[derive(States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    StartPage,
    Playing,
}

#[derive(Resource)]
pub struct GameInfo {
    pub curr_tetromino: Tetromino,
    pub next_tetromino: Tetromino,
    pub level: u32,
    pub points: f32,
}

impl GameInfo {
    pub fn new() -> Self {
        let mut initial_tetromino = Tetromino::default();
        GameInfo {
            curr_tetromino: initial_tetromino.clone(),
            next_tetromino: initial_tetromino.next().unwrap(),
            level: 0,
            points: 0.0,
        }
    }

    pub fn reset(&mut self) {
        let initial = &mut Tetromino::default();
        self.curr_tetromino = initial.clone();
        self.next_tetromino = initial.next().unwrap();
        self.level = 0;
        self.points = 0.0;
    }

    pub fn update_tetrominoes(&mut self) {
        let next_tetromino = self.next_tetromino.next().unwrap();

        self.curr_tetromino = self.next_tetromino.clone();
        self.next_tetromino = next_tetromino;
    }

    pub fn update_points(&mut self, value: f32) {
        self.points += value;
    }
}

pub fn update_level(mut game_info: ResMut<GameInfo>) {
    if game_info.points > exp(game_info.level as f32) {
        game_info.level += 1;
    }
}

pub fn update_points(mut game_info: ResMut<GameInfo>, mut event_reader: EventReader<RowCleaned>) {
    for _ in event_reader.read() {
        game_info.points += 10.0;
    }
}

pub fn reset_game_info(mut game_info: ResMut<GameInfo>) {
    game_info.reset();
}
