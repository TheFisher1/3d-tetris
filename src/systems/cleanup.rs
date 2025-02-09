use bevy::prelude::*;
use crate::game_state::{GameGrid, GridWall, Tetromino, Stopped, GRID_DEPTH, GRID_HEIGHT, GRID_WIDTH};

pub fn cleanup(
    mut commands: Commands,
    mut game_grid: ResMut<GameGrid>,
    tetromino_query: Query<(Entity, &Children), (With<Tetromino>, With<Stopped>)>,
) {
    // Check each y-level
    for y in 1..GRID_HEIGHT-1 {
        if game_grid.is_row_occupied(y) {
            println!("CLEANUP");
            game_grid.clear_row(y);
            for (entity, _) in tetromino_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}