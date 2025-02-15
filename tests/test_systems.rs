use bevy::input::InputPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use tetris::game::game_info::GameState;
use tetris::game::systems::setup::play_page::tetromino::check_is_game_over;
use tetris::game::{game_elements::*, state::game_info::GameInfo, systems::*};

use crate::utils::util_functions;
use crate::utils::util_functions::{gen_tetromino, get_or_find_transform};

#[test]
fn test_handle_despawn_event() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(GameGrid::new())
        .insert_resource(GameInfo::new())
        .add_event::<RowCleaned>()
        .add_systems(Update, (cleanup, handle_despawn_event_blocks));

    let level_to_fill = 1.0f32;

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

    let random_value_x = 4.0f32;
    let random_value_z = 2.0f32;

    app.world_mut().spawn((
        Transform::from_xyz(random_value_x, level_to_fill + 1.0, random_value_z),
        TetrominoBlock { id: 1 },
        Stopped,
    ));

    app.update();
    app.update();
    app.update();

    let remaining_blocks_at_level_to_fill = app
        .world_mut()
        .query::<(&TetrominoBlock, &Transform)>()
        .iter(&app.world())
        .filter(|(_, transform)| transform.translation.y == level_to_fill)
        .count();

    assert!(
        remaining_blocks_at_level_to_fill > 0,
        "The blocks that are over the row that has been removed must fall down"
    );

    app.world_mut()
        .resource_mut::<Events<RowCleaned>>()
        .send(RowCleaned(1));
}

#[test]
fn test_keyboard_movement() {
    let mut app = App::new();
    let mut time = Time::default();
    time.update();

    app.add_plugins(MinimalPlugins)
        .add_plugins(TransformPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(HierarchyPlugin)
        .insert_resource(time)
        .insert_resource(GameGrid::new())
        .insert_resource(ButtonInput::<KeyCode>::default())
        .add_systems(Update, keyboard_system);

    let tetromino_id = gen_tetromino(&mut app, (2.0, 5.0, 2.0), 0.01, 0.0001);

    let initial_transform = app.world().get::<Transform>(tetromino_id).unwrap().clone();

    for _ in 0..10 {
        app.update();
    }

    app.update();

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyW);

    for _ in 0..100 {
        app.update();
    }

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(KeyCode::KeyW);
    for _ in 0..100 {
        app.update();
    }

    let final_transform = app.world().get::<Transform>(tetromino_id).unwrap();

    assert_eq!(
        final_transform.translation.x,
        initial_transform.translation.x,
    );

    assert_eq!(
        final_transform.translation.z + BLOCK_SIZE,
        initial_transform.translation.z,
    )
}

#[test]
fn test_rotation_system() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .add_plugins(TransformPlugin)
        .add_plugins(HierarchyPlugin)
        .insert_resource::<Time>(Time::default())
        .insert_resource(GameGrid::new())
        .add_systems(Update, falling);

    let tetromino_id = app
        .world_mut()
        .spawn((
            Transform::from_xyz(2.0, 10.0, 2.0),
            Tetromino::default(),
            Falling {
                timer: Timer::from_seconds(0.01, TimerMode::Repeating),
            },
        ))
        .id();

    let _ = app
        .world_mut()
        .spawn((TetrominoBlock { id: 2 }, Transform::default()))
        .set_parent(tetromino_id)
        .id();

    let transform = util_functions::get_or_find_transform(&mut app, tetromino_id);
    let initial_y = transform.translation.y;

    for _ in 0..75 {
        app.update();
    }

    let transform = app.world().get::<Transform>(tetromino_id).unwrap();
    assert!(transform.translation.y < initial_y);
}

#[test]
fn test_game_over_detection() {
    let mut app = App::new();
    app.add_plugins(StatesPlugin)
        .add_plugins(MinimalPlugins)
        .insert_state(GameState::Playing)
        .insert_resource(GameGrid::new())
        .add_systems(Update, check_is_game_over);

    let mut grid = app.world_mut().resource_mut::<GameGrid>();
    for x in 1..GRID_WIDTH {
        for z in 1..GRID_DEPTH {
            grid.set_cell(x, GRID_HEIGHT - 1, z, 1);
            grid.set_cell(x, z, GRID_HEIGHT - 2, 1);
        }
    }

    app.update();

    app.world_mut()
        .spawn((
            Transform::from_xyz(2.0, (GRID_HEIGHT - 1) as f32, 2.0),
            Tetromino {
                id: 1,
                tetromino_type: TetrominoType::I,
            },
            Active,
            Falling {
                timer: Timer::from_seconds(FALL_TIME, TimerMode::Repeating),
            },
            MovementCooldown {
                timer: Timer::from_seconds(0.1, TimerMode::Once),
            },
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn(Tetromino {
                id: 2,
                tetromino_type: TetrominoType::L,
            });
        });

    app.update();
    app.update();
    app.update();

    assert_eq!(
        app.world().resource::<State<GameState>>().get(),
        &GameState::StartPage
    );
}

#[test]
fn test_falling_timer() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .add_plugins(TransformPlugin)
        .add_plugins(HierarchyPlugin)
        .insert_resource::<Time>(Time::default())
        .insert_resource(GameGrid::new())
        .add_systems(Update, falling);

    let timer: Timer = Timer::from_seconds(0.01, TimerMode::Repeating);
    let tetromino_id = app
        .world_mut()
        .spawn((
            Transform::from_xyz(2.0, 10.0, 2.0),
            Tetromino::default(),
            Falling { timer },
        ))
        .id();

    let _ = app
        .world_mut()
        .spawn((TetrominoBlock { id: 2 }, Transform::default()))
        .set_parent(tetromino_id)
        .id();

    let transform = get_or_find_transform(&mut app, tetromino_id);
    let initial_y = transform.translation.y;

    for _ in 0..100 {
        app.update();
    }

    let transform = app.world().get::<Transform>(tetromino_id).unwrap();
    assert!(transform.translation.y < initial_y,
            "New transform must be at least one block lower than previously, initial_y: {}, final_y: {}",
            initial_y, transform.translation.y);

    for _ in 0..250 {
        app.update();
    }

    let transform = get_or_find_transform(&mut app, tetromino_id);
    assert!(transform.translation.y < initial_y,
            "New transform must be at least one block lower than previously, initial_y: {}, final_y: {}",
            initial_y, transform.translation.y);
}

#[test]
fn test_repeating_timer() {
    let mut app = App::new();
    let mut time = Time::default();
    time.update();

    app.add_plugins(MinimalPlugins)
        .insert_resource(time)
        .insert_resource(GameGrid::new())
        .add_systems(Update, falling);

    let tetromino_id = gen_tetromino(&mut app, (2.0, 5.0, 2.0), 0.001, 0.0);

    let initial_y = app
        .world()
        .get::<Transform>(tetromino_id)
        .unwrap()
        .translation
        .y;

    // This should be enough to simulate 1.0 seconds
    for _ in 0..10 {
        app.update();
    }

    let transform = util_functions::get_or_find_transform(&mut app, tetromino_id);

    assert!(transform.translation.y < initial_y,
            "New transform must be at least one block lower than previously; initial_y: {}, final_y: {}",
            initial_y, transform.translation.y);

    for _ in 0..10 {
        app.update();
    }

    let transform = util_functions::get_or_find_transform(&mut app, tetromino_id);

    assert!(transform.translation.y < initial_y,
            "New transform must be at least one block lower than previously; initial_y: {}, final_y: {}",
            initial_y, transform.translation.y);
}

#[test]
fn test_basic_movement() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(TransformPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(HierarchyPlugin)
        .insert_resource(ButtonInput::<KeyCode>::default())
        .insert_resource::<Time>(Time::default())
        .insert_resource(GameGrid::new())
        .add_systems(Update, keyboard_system);

    let tetromino_id = gen_tetromino(&mut app, (2.0, 5.0, 2.0), FALL_TIME, 0.0);

    let initial_x = app
        .world()
        .get::<Transform>(tetromino_id)
        .unwrap()
        .translation
        .x;

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyD);
    app.update();

    let final_x = app
        .world()
        .get::<Transform>(tetromino_id)
        .unwrap()
        .translation
        .x;
    assert!(final_x > initial_x, "Tetromino should move right");
}
