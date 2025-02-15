use bevy::prelude::*;
use tetris::{Active, Falling, MovementCooldown, Tetromino, TetrominoBlock};

pub fn get_or_find_transform(mut app: &mut App, tetromino_id: Entity) -> Transform {
    match app.world().get::<Transform>(tetromino_id) {
        None => {
            *app.world_mut()
                .query::<(&TetrominoBlock, &Transform)>()
                .get_single(app.world())
                .unwrap()
                .1
        }
        Some(&transform) => transform,
    }
}

pub fn gen_tetromino(
    mut app: &mut App,
    position: (f32, f32, f32),
    fall_time: f32,
    movement_cooldown_time: f32,
) -> Entity {
    let tetromino_id = app
        .world_mut()
        .spawn((
            Transform::from_xyz(position.0, position.1, position.2),
            Tetromino::default(),
            MovementCooldown {
                timer: Timer::from_seconds(movement_cooldown_time, TimerMode::Once),
            },
            Falling {
                timer: Timer::from_seconds(fall_time, TimerMode::Repeating),
            },
            Active,
        ))
        .id();

    let _ = app
        .world_mut()
        .spawn((TetrominoBlock { id: 1 }, Transform::default()))
        .set_parent(tetromino_id);

    tetromino_id
}
