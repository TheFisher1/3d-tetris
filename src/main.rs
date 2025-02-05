mod components;
mod systems;
mod resources;
mod constants;

use bevy::prelude::*;
use components::*;
use systems::*;
use resources::*;
use constants::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameGrid::new())
        .add_systems(Startup, (setup, spawn_grid))
        .add_systems(Update, (
            falling,
            rotate_camera,
            move_tetromino,
            update_grid_state,
        ))
        .run();
} 

fn update_grid_state(
    mut game_grid: ResMut<GameGrid>,
    query: Query<&Transform, Added<Stopped>>,
) {
    for transform in query.iter() {
        println!("\nPlacing stopped blocks at:");
        for block_pos in get_tetromino_block_positions(transform) {
            let (grid_x, grid_y, grid_z) = get_grid_position(block_pos);
            
            println!(
                "World pos: ({:.1}, {:.1}, {:.1}), Grid pos: ({}, {}, {})",
                block_pos.x, block_pos.y, block_pos.z, grid_x, grid_y, grid_z
            );
            
            game_grid.set_cell(grid_x, grid_y, grid_z, true);
        }
    }
} 