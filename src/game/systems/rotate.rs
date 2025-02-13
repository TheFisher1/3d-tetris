use crate::game::game_state::{Falling, GameGrid, RotateButton, Tetromino, ROTATION_DEGREES};
use crate::game::systems::is_valid_position;
use bevy::math::Dir3;
use bevy::prelude::{Button, Changed, Children, Interaction, Query, Res, Transform, With, Without};

use super::is_valid_position_tetromino;

pub fn rotate_system(
    mut interaction_query: Query<
        (&Interaction, &RotateButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut piece_query: Query<(&mut Transform, &Children), (With<Tetromino>, With<Falling>)>,
    transform_query: Query<&Transform, Without<Tetromino>>,
    game_grid: Res<GameGrid>,
) {
    if let Ok((interaction, button)) = interaction_query.get_single_mut() {
        if *interaction == Interaction::Pressed {
            for (mut transform, children) in piece_query.iter_mut() {
                handle_rotate(
                    button,
                    &mut transform,
                    children,
                    &transform_query,
                    game_grid,
                );
                return;
            }
        }
    }
}

fn handle_rotate(
    button: &RotateButton,
    transform: &mut Transform,
    children: &Children,
    transform_query: &Query<&Transform, Without<Tetromino>>,
    game_grid: Res<GameGrid>,
) {
    match button {
        RotateButton::Up => {
            rotate_transform(
                transform,
                children,
                transform_query,
                game_grid,
                ROTATION_DEGREES.to_radians(),
                Dir3::Z,
            );
        }
        RotateButton::Left => {
            rotate_transform(
                transform,
                children,
                transform_query,
                game_grid,
                ROTATION_DEGREES.to_radians(),
                Dir3::Y,
            );
        }
        RotateButton::Right => {
            rotate_transform(
                transform,
                children,
                transform_query,
                game_grid,
                -ROTATION_DEGREES.to_radians(),
                Dir3::Y,
            );
        }
        RotateButton::Down => {
            rotate_transform(
                transform,
                children,
                transform_query,
                game_grid,
                -ROTATION_DEGREES.to_radians(),
                Dir3::Z,
            );
        }
    }
}

fn rotate_transform(
    transform: &mut Transform,
    children: &Children,
    transform_query: &Query<&Transform, Without<Tetromino>>,
    game_grid: Res<GameGrid>,
    rotation_degrees_radians: f32,
    rotation_axis: Dir3,
) {
    let mut new_transform = transform.clone();
    new_transform.rotate_axis(rotation_axis, rotation_degrees_radians);

    if !is_valid_position_tetromino(&new_transform, &game_grid, children, transform_query) {
        return;
    }

    *transform = new_transform;
}
