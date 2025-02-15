use crate::game::game_elements::GameCamera;
use crate::game::game_elements::GameGrid;
use crate::game::game_elements::{Tetromino, TetrominoBlock};
use crate::game::systems::setup::play_page::grid::Grid;
use crate::game::systems::setup::play_page::play_page::RotationControls;
use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Query, ResMut, With};

pub fn game_despawn(
    mut commands: Commands,
    query_grid: Query<Entity, With<Grid>>,
    query_tetrominoes: Query<Entity, With<Tetromino>>,
    query_tetromino_blocks: Query<Entity, With<TetrominoBlock>>,
    query_buttons: Query<Entity, With<RotationControls>>,
    camera3d: Query<Entity, With<GameCamera>>,
    mut game_grid: ResMut<GameGrid>,
) {
    query_grid.iter().for_each(|e| {
        commands.entity(e).despawn_recursive();
    });

    query_tetrominoes.iter().for_each(|e| {
        commands.entity(e).despawn_recursive();
    });

    query_tetromino_blocks.iter().for_each(|e| {
        commands.entity(e).despawn_recursive();
    });

    query_buttons.iter().for_each(|e| {
        commands.entity(e).despawn_recursive();
    });

    game_grid.reset();

    if let Ok(entity) = camera3d.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
