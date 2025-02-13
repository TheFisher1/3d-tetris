use bevy::app::App;
use bevy::app::Plugin;
use bevy::app::Startup;

use crate::game::game_elements::GameGrid;
use crate::game::state::game_state::{GameState, GameStatus};
use crate::game::systems::cleanup;
use crate::game::systems::falling;
use crate::game::systems::falling_blocks;
use crate::game::systems::handle_despawn_event;
use crate::game::systems::handle_despawn_event_blocks;
use crate::game::systems::keyboard_system;
use crate::game::systems::rotate_camera;
use crate::game::systems::rotate_system;
use crate::game::systems::spawn::grid::spawn_grid;
use crate::game::systems::spawn::setup::setup;
use crate::game::systems::spawn::tetromino::spawn_new_tetromino;
use crate::game::systems::update_grid_state;
use crate::game::systems::RowCleaned;
use bevy::prelude::*;

pub struct TetrisPlugin;

impl Plugin for TetrisPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameGrid::new())
            .insert_state(GameStatus::Playing)
            .insert_resource(GameState::new(0, 0.0))
            .add_event::<RowCleaned>()
            .add_systems(Startup, (setup, spawn_grid))
            .add_systems(
                Update,
                (
                    rotate_camera,
                    update_grid_state,
                    rotate_system,
                    handle_despawn_event,
                    falling_blocks,
                    handle_despawn_event_blocks,
                    falling,
                    cleanup,
                    spawn_new_tetromino,
                    keyboard_system,
                )
                    .run_if(in_state(GameStatus::Playing)),
            );
    }
}
