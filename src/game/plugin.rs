use bevy::app::App;
use bevy::app::Plugin;
use bevy::app::Startup;

use bevy::prelude::*;
use crate::game::game_state::GameGrid;

use crate::game::systems::falling;
use crate::game::systems::falling_blocks;
use crate::game::systems::handle_despawn_event;
use crate::game::systems::cleanup;
use crate::game::systems::handle_despawn_event_blocks;
use crate::game::systems::keyboard_system;
use crate::game::systems::rotate_camera;
use crate::game::systems::rotate_system;
use crate::game::systems::setup::setup;
use crate::game::systems::spawn_grid;
use crate::game::systems::spawn_new_tetromino;
use crate::game::systems::update_grid_state;
use crate::game::systems::RowCleaned;

pub struct TetrisPlugin;

impl Plugin for TetrisPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameGrid::new())
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
                ),
            )
            .add_systems(
                Update,
                (falling, cleanup, spawn_new_tetromino, keyboard_system).chain(),
            );
    }
}
