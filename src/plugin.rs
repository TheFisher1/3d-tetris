use bevy::app::{App, Plugin, Startup};
use bevy::prelude::*;


use crate::game_state::resources::GameGrid;
use crate::game_state::resources::*;
use crate::systems::setup::{setup, spawn_new_tetromino};
use crate::systems::{falling, cleanup, keyboard_system, rotate_camera, rotate_system, spawn_grid, update_grid_state};

pub struct TetrisPlugin;

impl Plugin for TetrisPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameGrid::new())
        .add_systems(Startup, (setup, spawn_grid))
        .add_systems(Update, (
        rotate_camera,
        update_grid_state,
        rotate_system,
        ))
        .add_systems(Update, (
        falling,
        cleanup,
        spawn_new_tetromino,
        keyboard_system,
        ).chain());
    }
}