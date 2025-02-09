use bevy::prelude::*;
use crate::game_state::{GameGrid, GridWall, Tetromino, Stopped, GRID_DEPTH, GRID_HEIGHT, GRID_WIDTH};

pub fn cleanup(
    mut commands: Commands,
    mut game_grid: ResMut<GameGrid>,
    tetromino_query: Query<(Entity, &Transform, &Children), (With<Tetromino>, With<Stopped>)>,
    transform_query: Query<&Transform>,
) {
    for y in 1..GRID_HEIGHT-1 {
        if game_grid.is_row_occupied(y) {
            game_grid.clear_row(y);

            for (_, tetromino_transform, children) in tetromino_query.iter() {
                // Iterate over each child (block) of the tetromino
                for &child in children.iter() {
                    if let Ok(child_transform) = transform_query.get(child) {
                        // Get the block's local position relative to the tetromino
                        let local_position = child_transform.translation;

                        // Apply the tetromino's rotation to the block's local position
                        let rotated_position = tetromino_transform.rotation * local_position;

                        // Now calculate the block's world position by adding the tetromino's world position
                        let block_world_position = tetromino_transform.translation + rotated_position;

                        // Check if the block's y position matches the desired y
                        if block_world_position.y == y as f32 {
                            commands.entity(child).despawn();
                            println!("Block at y = {} is at the desired position!", block_world_position.y);
                            // You can add logic to handle the block here (e.g., clear it if it's at the desired row)
                        }
                    }
                }
            }
            // for (entity, _) in tetromino_query.iter() {
            //     commands.entity(entity).despawn_recursive();
            // }
        }
    }
}