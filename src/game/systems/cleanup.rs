use crate::game::game_state::{GameGrid, Stopped, TetrominoBlock, GRID_HEIGHT};
use bevy::prelude::*;

#[derive(Event)]
pub struct RowCleaned(pub u32);

pub fn cleanup(
    mut commands: Commands,
    mut game_grid: ResMut<GameGrid>,
    mut event_writer: EventWriter<RowCleaned>,
    tetromino_query: Query<(Entity, &Transform), (With<TetrominoBlock>, With<Stopped>)>,
) {
    for y in 1..GRID_HEIGHT - 1 {
        if game_grid.is_row_occupied(y) {
            game_grid.clear_row(y);

            for (entity, transform) in tetromino_query.iter() {
                let block_y = transform.translation.y.round() as u32;
                if block_y == y {
                    commands.entity(entity).despawn();
                }
            }

            event_writer.send(RowCleaned(y));
        }
    }
}