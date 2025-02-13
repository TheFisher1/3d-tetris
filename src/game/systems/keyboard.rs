use crate::game::game_elements::{
    Active, Falling, GameGrid, MovementCooldown, Tetromino, BLOCK_SIZE,
};
use crate::game::systems::is_valid_position;
use bevy::input::ButtonInput;
use bevy::prelude::*;

use super::is_valid_position_tetromino;

pub fn keyboard_system(
    time: Res<Time>,
    game_grid: Res<GameGrid>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (&mut Transform, &mut MovementCooldown, &Tetromino, &Children),
        (With<Falling>, With<Active>),
    >,
    transform_query: Query<&Transform, Without<Tetromino>>,
) {
    if let Ok((mut transform, mut cooldown, _, children)) = query.get_single_mut() {
        if keyboard.just_pressed(KeyCode::Space) {
            let mut test_transform = transform.clone();
            let mut highest_valid_y = test_transform.translation.y;

            let mut highest_valid_y_reached = true;
            while test_transform.translation.y > 0.0 && highest_valid_y_reached {
                test_transform.translation.y -= BLOCK_SIZE;
                if !is_valid_position_tetromino(
                    &test_transform,
                    &game_grid,
                    children,
                    &transform_query,
                ) {
                    test_transform.translation.y += BLOCK_SIZE;
                    highest_valid_y = test_transform.translation.y;
                    highest_valid_y_reached = false;
                }
            }

            transform.translation.y = highest_valid_y;
            return;
        }

        cooldown.timer.tick(time.delta());

        if cooldown.timer.finished() {
            let mut moved = false;
            let mut new_transform = transform.clone();

            if keyboard.just_pressed(KeyCode::KeyA) {
                new_transform.translation.x -= BLOCK_SIZE;
                moved = true;
            }

            if keyboard.just_pressed(KeyCode::KeyD) {
                new_transform.translation.x += BLOCK_SIZE;
                moved = true;
            }

            if keyboard.just_pressed(KeyCode::KeyW) {
                new_transform.translation.z -= BLOCK_SIZE;
                moved = true;
            }

            if keyboard.just_pressed(KeyCode::KeyS) {
                new_transform.translation.z += BLOCK_SIZE;
                moved = true;
            }

            if moved {
                if is_valid_position_tetromino(
                    &new_transform,
                    &game_grid,
                    children,
                    &transform_query,
                ) {
                    *transform = new_transform;
                }

                cooldown.timer.reset();
            }
        }
    }
}
