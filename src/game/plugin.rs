use bevy::app::App;
use bevy::app::Plugin;
use bevy::app::Startup;

use crate::game::game_elements::GameGrid;
use crate::game::game_info::{update_level, update_points};
use crate::game::state::game_info::{reset_game_info, GameInfo, GameState};
use crate::game::systems::cleanup_row;
use crate::game::systems::cleanup_home_page::cleanup_home_page;
use crate::game::systems::{
    falling, falling_blocks, handle_despawn_event, 
    handle_despawn_event_blocks,
    keyboard_system,
    rotate_camera,
    rotate_system
};
use crate::game::systems::setup::play_page::{
    cleanup_play_page::cleanup_play_page,
    grid::spawn_grid, 
    play_page::setup,
    tetromino::check_is_game_over,
    tetromino::spawn_new_tetromino
};
use crate::game::systems::{
    update_grid_state, 
    util::add_sound,
    RowCleaned, {handle_start_button, setup_home_page}
};
use crate::game::systems::util::add_image;
use bevy::prelude::*;

pub struct TetrisPlugin;

impl Plugin for TetrisPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameGrid::new())
            .insert_state(GameState::StartPage)
            .add_systems(Startup, (add_sound, add_image))
            .add_systems(
                OnEnter(GameState::StartPage),
                setup_home_page.after(cleanup_play_page),
            )
            .add_systems(
                Update,
                handle_start_button.run_if(in_state(GameState::StartPage)),
            )
            .add_systems(OnExit(GameState::StartPage), cleanup_home_page)
            .add_systems(
                OnEnter(GameState::Playing),
                (setup, spawn_grid).chain().after(cleanup_home_page),
            )
            .insert_resource(GameInfo::new())
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
                    cleanup_row,
                    spawn_new_tetromino,
                    keyboard_system,
                    check_is_game_over,
                    update_points,
                    update_level,
                )
                    .after(spawn_grid)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                OnExit(GameState::Playing),
                (reset_game_info, cleanup_play_page).chain(),
            );
    }
}
