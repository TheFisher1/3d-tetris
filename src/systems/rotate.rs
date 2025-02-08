use bevy::math::{Dir3, Quat, Vec3};
use bevy::prelude::{Button, Changed, Interaction, Query, Res, Transform, With, Without};
use crate::game_state::{Falling, GameGrid, RotateButton, Tetromino, ROTATION_DEGREES, ROTATION_SPEED};
use crate::systems::is_valid_position;

pub fn rotate_system(
    mut interaction_query: Query<(&Interaction, &RotateButton), (Changed<Interaction>, With<Button>)>,
    mut piece_query: Query<&mut Transform, (With<Tetromino>, With<Falling>)>,
    game_grid: Res<GameGrid>
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            let mut transform = piece_query.single_mut();
                handle_rotate(button, &mut *transform, game_grid);
                return;
            }
    }
}

fn handle_rotate(
    button: &RotateButton,
    transform: &mut Transform,
    game_grid: Res<GameGrid>,
) {
    match button {
        RotateButton::Up => {
            rotate_transform(&mut *transform, game_grid, ROTATION_DEGREES.to_radians(), Dir3::Z);
            return;
        }
        RotateButton::Left => {
            rotate_transform(&mut *transform, game_grid, ROTATION_DEGREES.to_radians(), Dir3::Y);
            return;
        }
        RotateButton::Right => {
            rotate_transform(&mut *transform, game_grid, -ROTATION_DEGREES.to_radians(), Dir3::Y);
            return;
        }
        RotateButton::Down => {
            rotate_transform(&mut *transform, game_grid, -ROTATION_DEGREES.to_radians(), Dir3::Z);
            return;
        }
    }
}

fn rotate_transform(
    transform: &mut Transform,
    game_grid: Res<GameGrid>,
    rotation_degrees_radians: f32,
    rotation_axis: Dir3,
) {
    let mut new_transform = transform.clone();
    new_transform.rotate_axis(rotation_axis, rotation_degrees_radians);

    if !is_valid_position(&new_transform, &game_grid) {
        return;
    }

    *transform = new_transform;
}