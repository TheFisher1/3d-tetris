use bevy::app::App;
use bevy::app::Plugin;
use bevy::app::Startup;

use crate::game::game_elements::GameGrid;
use crate::game::state::game_state::{GameInfo, GameState};
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
use crate::game::systems::{handle_start_button, setup_home_page};
use crate::game::systems::cleanup_home_page::cleanup_home_page;

pub struct TetrisPlugin;

impl Plugin for TetrisPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameGrid::new())
            .insert_state(GameState::StartPage)
            .add_systems(Startup, setup_home_page)
            .add_systems(OnEnter(GameState::StartPage), setup_home_page)
            .add_systems(Update, handle_start_button.run_if(in_state(GameState::StartPage)))
            .add_systems(OnExit(GameState::StartPage), cleanup_home_page)
            .add_systems(OnEnter(GameState::Playing), (setup, spawn_grid).chain())
            .insert_resource(GameInfo::new(0, 0.0))
            .add_event::<RowCleaned>()
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
                    .after(spawn_grid)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
