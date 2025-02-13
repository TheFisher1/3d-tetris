use crate::game::game_elements::*;
use bevy::prelude::*;

pub fn update_grid_state(
    mut game_grid: ResMut<GameGrid>,
    query: Query<(&Transform, &TetrominoBlock), Added<Stopped>>,
) {
    for (transform, block) in query.iter() {
        let rotated_translation = transform.rotation * transform.translation;
        let (grid_x, grid_y, grid_z) = (
            rotated_translation.x,
            rotated_translation.y,
            rotated_translation.z,
        );
        game_grid.set_cell(grid_x as i32, grid_y as i32, grid_z as i32, block.id);
    }
}
