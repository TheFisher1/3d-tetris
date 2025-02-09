use bevy::prelude::*;
use crate::game_state::{GameGrid, GridWall, Tetromino, Stopped, GRID_DEPTH, GRID_HEIGHT, GRID_WIDTH};

#[derive(Event)]
pub struct RowCleaned(pub u32);

pub fn cleanup(
    mut commands: Commands,
    mut game_grid: ResMut<GameGrid>,
    mut event_writer: EventWriter<RowCleaned>,
    tetromino_query: Query<(Entity, &Transform, &Children), (With<Tetromino>, With<Stopped>)>,
    transform_query: Query<&Transform>,
) {
    for y in 1..GRID_HEIGHT-1 {
        if game_grid.is_row_occupied(y) {
            game_grid.clear_row(y);

            for (_, tetromino_transform, children) in tetromino_query.iter() {
                for &child in children.iter() {
                    if let Ok(child_transform) = transform_query.get(child) {
                        let block_world_position = calculate_world_position(child_transform, tetromino_transform);

                        if block_world_position.y == y as f32 {
                            commands.entity(child).despawn();
                            println!("Block at y = {} is at the desired position!", block_world_position.y);
                        }
                    }
                }
            }

            event_writer.send(RowCleaned(y));
        }
    }
}

pub fn calculate_world_position(child_transform: &Transform, tetromino_transform: &Transform) -> Vec3 {
    let local_position = child_transform.translation;
    let rotated_position = tetromino_transform.rotation * local_position;

    tetromino_transform.translation + rotated_position
}