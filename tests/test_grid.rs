use crate::utils::util_functions::{gen_tetromino, get_or_find_transform};
use bevy::prelude::*;
use tetris::game::game_elements::GameGrid;
use tetris::game::systems::get_grid_position;
use tetris::game::systems::movement::is_valid_position;
use tetris::game::{GRID_DEPTH, GRID_WIDTH};
use tetris::game_info::GameInfo;
use tetris::systems::{falling, update_grid_state, RowCleaned};
use tetris::GRID_HEIGHT;

#[test]
fn test_is_valid_position() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    let game_grid = GameGrid::new();

    let parent_entity = app
        .world_mut()
        .spawn((Transform::from_xyz(1.0, 1.0, 1.0),))
        .id();

    app.insert_resource(GameGrid::new());

    let transform = app.world().get::<Transform>(parent_entity).unwrap();

    assert!(is_valid_position(
        &(transform.rotation * transform.translation),
        &game_grid,
    ));

    let new_parent_entity = app
        .world_mut()
        .spawn((Transform::from_xyz(50.0, 1.0, 1.0),))
        .id();

    let transform = app.world().get::<Transform>(new_parent_entity).unwrap();
    assert!(!is_valid_position(
        &(transform.rotation * transform.translation),
        &game_grid
    ));
}

#[test]
fn test_grid_position_conversion_rounding() {
    let position = Vec3::new(1.4, 2.6, 3.5);
    assert_eq!(get_grid_position(position), (1, 3, 4));
}

#[test]
fn test_game_grid_creation() {
    let grid = GameGrid::new();
    assert!(grid.is_cell_empty(1, 1, 1));
}

#[test]
fn test_game_grid_set_cell() {
    let mut grid = GameGrid::new();
    grid.set_cell(1, 1, 1, 1);
    assert!(!grid.is_cell_empty(1, 1, 1));
}

#[test]
fn test_game_grid_clear_row() {
    let mut grid = GameGrid::new();
    for x in 1..GRID_WIDTH {
        for z in 1..GRID_DEPTH {
            grid.set_cell(x, 1, z, 1);
        }
    }

    assert!(grid.is_row_occupied(1));
    grid.clear_row(1);
    assert!(!grid.is_row_occupied(1));
}

#[test]
fn test_game_grid_reset() {
    let mut grid = GameGrid::new();

    grid.set_cell(1, 1, 1, 1);
    grid.set_cell(2, 2, 2, 1);
    grid.reset();
    assert!(grid.is_cell_empty(1, 1, 1));
    assert!(grid.is_cell_empty(2, 2, 2));
}

#[test]
fn test_invalid_coordinates() {
    let grid = GameGrid::new();

    assert!(!grid.is_cell_empty(-1, 0, 0));
    assert!(!grid.is_cell_empty(0, -1, 0));
    assert!(!grid.is_cell_empty(0, 0, -1));

    assert!(!grid.is_cell_empty(GRID_WIDTH, 0, 0));
    assert!(!grid.is_cell_empty(0, GRID_HEIGHT, 0));
    assert!(!grid.is_cell_empty(0, 0, GRID_DEPTH));
}

#[test]
fn test_update_grid() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(GameGrid::new())
        .insert_resource(GameInfo::new())
        .add_event::<RowCleaned>()
        .add_systems(Update, (falling, update_grid_state));

    let tetromino_id = gen_tetromino(&mut app, (2.0, 5.0, 2.0), 0.0, 0.01);

    for _ in 0..50 {
        app.update();
    }

    let transform = get_or_find_transform(&mut app, tetromino_id);

    let game_grid = app.world().resource::<GameGrid>();
    assert!(!game_grid.is_cell_empty(2, 1, 2))
}
