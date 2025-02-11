use bevy::prelude::*;
use crate::game_state::{Active, Falling, GameGrid, MovementCooldown, Stopped, Tetromino, TetrominoBlock, BLOCK_SIZE, GRID_DEPTH, GRID_HEIGHT, GRID_WIDTH, ROTATION_DEGREES};
use crate::systems::{RowCleaned};
use crate::game_state::constants::FALL_TIME;

pub fn falling(
    time: Res<Time>,
    game_grid: Res<GameGrid>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Falling, &Tetromino, &Children)>,
    transform_query: Query<&Transform, Without<Tetromino>>,
) {
    for (entity, mut transform, mut falling, tetromino, children) in query.iter_mut() {
        falling.timer.tick(time.delta());

        if !falling.timer.just_finished() {
            return;
        }

        let mut new_transform = transform.clone();
        new_transform.translation.y -= BLOCK_SIZE;
            
        if !is_valid_position(&new_transform, &game_grid, children, &transform_query) {
            // Remove parent entity components
            commands.entity(entity).remove::<Falling>().remove::<Active>();

            // Convert each child block into an independent entity
            for &child in children.iter() {
                if let Ok(child_transform) = transform_query.get(child) {
                    let world_pos = transform.transform_point(child_transform.translation);
                    
                    commands.entity(child)
                        .remove_parent()
                        .insert(Transform::from_translation(world_pos))
                        .insert(Stopped)
                        .insert(TetrominoBlock { id: tetromino.id });
                }
            }

            // Despawn the parent entity after converting children
            commands.entity(entity).despawn();
        } else {
            *transform = new_transform;
        }
    }
}

pub fn falling_blocks(
    time: Res<Time>,
    game_grid: Res<GameGrid>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Falling, &TetrominoBlock, &Children)>,
) {
    for (entity, mut transform, mut falling, tetromino, children) in query.iter_mut() {
        falling.timer.tick(time.delta());
        if !falling.timer.finished() {
            return;
        }

        let mut new_transform = transform.clone();
        new_transform.translation.y -= BLOCK_SIZE;

        if !is_valid_position_block(&new_transform, &game_grid) {
            commands.entity(entity)
                .remove::<Falling>()
                .insert(Stopped);
        } else {
            *transform = new_transform;
        }
    }
}

fn is_valid_position_block(transform: &Transform, game_grid: &Res<GameGrid>) -> bool {
    transform.translation.x < GRID_WIDTH as f32 && transform.translation.y < GRID_HEIGHT as f32 && transform.translation.z < GRID_DEPTH as f32
    && transform.translation.z > 0f32 && transform.translation.y > 0.0f32 && transform.translation.z > 0.0f32 &&
        game_grid.is_cell_empty(transform.translation.x as i32, transform.translation.y as i32, transform.translation.z as i32)
}

pub fn handle_despawn_event(
        mut commands: Commands,
        mut event_reader: EventReader<RowCleaned>,
        query: Query<(Entity, &Transform), (With<Tetromino>, With<Stopped>)>,
    ) {
        for event in event_reader.read() {
            let row_number = event.0;

            for (tetromino, transform) in query.iter() {
              if transform.translation.y == (row_number + 1) as f32 {
                  commands.entity(tetromino).remove::<Stopped>().insert(Falling { timer: Timer::from_seconds(FALL_TIME, TimerMode::Repeating) });
              }
            }
        }
    }

pub fn is_valid_position(
    transform: &Transform, 
    game_grid: &GameGrid,
    children: &Children,
    transform_query: &Query<&Transform, Without<Tetromino>>,
) -> bool {
    for &child in children.iter() {
        if let Ok(child_transform) = transform_query.get(child) {
            let world_pos = transform.transform_point(child_transform.translation);
            let (grid_x, grid_y, grid_z) = get_grid_position(world_pos);

            // Check boundaries
            if grid_x <= 0 || grid_x >= GRID_WIDTH as i32 ||
               grid_y <= 0 || grid_y >= GRID_HEIGHT as i32 ||
               grid_z <= 0 || grid_z >= GRID_DEPTH as i32 {
                return false;
            }

            // Check collision with other blocks
            if !game_grid.is_cell_empty(grid_x, grid_y, grid_z) {
                return false;
            }
        }
    }
    true
}

pub fn get_grid_position(world_pos: Vec3) -> (i32, i32, i32) {
    let grid_x = world_pos.x.round() as i32;
    let grid_y = world_pos.y.round() as i32;
    let grid_z = world_pos.z.round() as i32;
    (grid_x, grid_y, grid_z)
}