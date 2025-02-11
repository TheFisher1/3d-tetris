use bevy::app::{App, Plugin, Startup};
use bevy::prelude::*;


use crate::game_state::resources::GameGrid;
use crate::game_state::resources::*;
use crate::systems::setup::{setup, spawn_new_tetromino};
use crate::systems::{falling, cleanup, rotate_camera, rotate_system, spawn_grid, update_grid_state, RowCleaned, handle_despawn_event};
use crate::systems::keyboard_system;

pub struct TetrisPlugin;

impl Plugin for TetrisPlugin {
    fn build(&self, app: &mut App) {
            app
            .insert_resource(GameGrid::new())
            .add_event::<RowCleaned>()
        .add_systems(Startup, (setup, spawn_grid))
        .add_systems(Update, (
        rotate_camera,
        update_grid_state,
        rotate_system,
        handle_despawn_event
        ))
        .add_systems(Update, (
        falling,
        cleanup,
        spawn_new_tetromino,
        keyboard_system,
        ).chain());
    }
}