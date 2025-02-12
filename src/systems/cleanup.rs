use bevy::prelude::*;
use crate::game_state::{GameGrid, GridWall, TetrominoBlock, Stopped, GRID_DEPTH, GRID_HEIGHT, GRID_WIDTH};

#[derive(Event)]
pub struct RowCleaned(pub u32);

pub fn cleanup(
    mut commands: Commands,
    mut game_grid: ResMut<GameGrid>,
    mut event_writer: EventWriter<RowCleaned>,
    tetromino_query: Query<(Entity, &Transform), (With<TetrominoBlock>, With<Stopped>)>,
) {
    for y in 1..GRID_HEIGHT-1 {
        if game_grid.is_row_occupied(y) {
            game_grid.clear_row(y);

            // Despawn all blocks in this row
            for (entity, transform) in tetromino_query.iter() {
                let block_y = transform.translation.y.round() as u32;
                if block_y == y {
                    commands.entity(entity).despawn();
                }
            }

            // Send event for blocks above to fall
            event_writer.send(RowCleaned(y));
        }
    }
}

pub fn calculate_world_position(child_transform: &Transform, tetromino_transform: &Transform) -> Vec3 {
    let local_position = child_transform.translation;
    let rotated_position = tetromino_transform.rotation * local_position;

    tetromino_transform.translation + rotated_position
}