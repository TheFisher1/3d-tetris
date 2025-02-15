use bevy::app::{App, Update};
use bevy::prelude::Transform;
use bevy::MinimalPlugins;
use tetris::game::state::game_info::GameInfo;
use tetris::game_info::{update_level, update_points};
use tetris::systems::{cleanup, RowCleaned, cleanup_row};
use tetris::{GameGrid, Stopped, TetrominoBlock, GRID_DEPTH, GRID_WIDTH};

#[test]
fn test_game_info_creation() {
    let game_info = GameInfo::new();
    assert_eq!(game_info.level, 0);
    assert_eq!(game_info.points, 0.0);
}

#[test]
fn test_game_info_set_points() {
    let mut game_info = GameInfo::new();
    game_info.update_points(10.0);
    assert_eq!(game_info.points, 10.0);
}

#[test]
fn test_game_info_update_tetrominoes() {
    let mut game_info = GameInfo::new();

    let initial_next = game_info.next_tetromino.clone();
    game_info.update_tetrominoes();
    assert_eq!(game_info.curr_tetromino.id, initial_next.id);
    assert_ne!(game_info.next_tetromino.id, initial_next.id);
}

#[test]
fn test_game_info_reset() {
    let mut game_info = GameInfo::new();
    game_info.update_points(100.0);
    game_info.reset();
    assert_eq!(game_info.points, 0.0);
    assert_eq!(game_info.level, 0);
}

#[test]
fn test_game_info_update_points() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(GameGrid::new())
        .insert_resource(GameInfo::new())
        .add_event::<RowCleaned>()
        .add_systems(Update, (cleanup_row, update_points));

    let level_to_fill = 1.0f32;

    let game_info = app.world().resource::<GameInfo>();
    assert_eq!(
        game_info.points, 0.0,
        "Game points must be 0.0 at the beginning"
    );

    for x in 1..GRID_WIDTH {
        for z in 1..GRID_DEPTH {
            app.world_mut().spawn((
                Transform::from_xyz(x as f32, level_to_fill, z as f32),
                TetrominoBlock { id: 1 },
                Stopped,
            ));
            app.world_mut()
                .resource_mut::<GameGrid>()
                .set_cell(x, level_to_fill as i32, z, 1);
        }
    }

    app.update();
    app.update();
    app.update();

    let game_info = app.world().resource::<GameInfo>();
    assert!(
        game_info.points > 0.0,
        "Game points must grow when a row is cleared, current_points: {}",
        game_info.points
    );
}

#[test]
fn test_game_info_update_level() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(GameGrid::new())
        .insert_resource(GameInfo::new())
        .add_event::<RowCleaned>()
        .add_systems(Update, (cleanup_row, update_level));

    let level_to_fill = 1.0f32;

    let game_info = app.world().resource::<GameInfo>();
    assert_eq!(
        game_info.points, 0.0,
        "Game points must be 0.0 at the beginning"
    );

    app.world_mut().resource_mut::<GameInfo>().points += 10.0;

    for x in 1..GRID_WIDTH {
        for z in 1..GRID_DEPTH {
            app.world_mut().spawn((
                Transform::from_xyz(x as f32, level_to_fill, z as f32),
                TetrominoBlock { id: 1 },
                Stopped,
            ));
            app.world_mut()
                .resource_mut::<GameGrid>()
                .set_cell(x, level_to_fill as i32, z, 1);
        }
    }

    app.update();
    app.update();
    app.update();

    let game_info = app.world().resource::<GameInfo>();
    assert!(
        game_info.level > 0,
        "Player must level up when he has enough points, current_points: {}",
        game_info.points
    );
}
