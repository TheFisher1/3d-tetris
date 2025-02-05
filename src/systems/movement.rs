use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::constants::*;

pub fn move_tetromino(
    time: Res<Time>,
    game_grid: Res<GameGrid>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut MovementCooldown), With<Tetromino>>,
) {
    let (mut transform, mut cooldown) = query.single_mut();
    
    // Add debug logging
    if keyboard.just_pressed(KeyCode::A) || 
       keyboard.just_pressed(KeyCode::D) || 
       keyboard.just_pressed(KeyCode::W) || 
       keyboard.just_pressed(KeyCode::S) {
        println!("Movement key pressed! Current position: {:?}", transform.translation);
    }

    cooldown.timer.tick(time.delta());

    if cooldown.timer.finished() {
        let mut moved = false;
        let mut new_transform = transform.clone();

        // Movement controls - move by exactly one block
        if keyboard.just_pressed(KeyCode::A) {
            new_transform.translation.x -= BLOCK_SIZE;
            moved = true;
        }
        if keyboard.just_pressed(KeyCode::D) {
            new_transform.translation.x += BLOCK_SIZE;
            moved = true;
        }
        if keyboard.just_pressed(KeyCode::W) {
            new_transform.translation.z -= BLOCK_SIZE;
            moved = true;
        }
        if keyboard.just_pressed(KeyCode::S) {
            new_transform.translation.z += BLOCK_SIZE;
            moved = true;
        }

        // Rotation controls
        if keyboard.just_pressed(KeyCode::Q) {
            new_transform.rotate_y(ROTATION_DEGREES.to_radians());
            // After rotation, ensure blocks are grid-aligned
            new_transform.translation = new_transform.translation.round();
            moved = true;
        }
        if keyboard.just_pressed(KeyCode::E) {
            new_transform.rotate_y(-ROTATION_DEGREES.to_radians());
            // After rotation, ensure blocks are grid-aligned
            new_transform.translation = new_transform.translation.round();
            moved = true;
        }

        if moved {
            if is_valid_position(&new_transform, &game_grid) {
                *transform = new_transform;
            }
            cooldown.timer.reset();
        }
    }
}

pub fn falling(
    time: Res<Time>,
    game_grid: Res<GameGrid>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Falling), With<Tetromino>>,
) {
    for (entity, mut transform, mut falling) in query.iter_mut() {
        falling.timer.tick(time.delta());

        if falling.timer.just_finished() {
            let mut new_transform = transform.clone();
            new_transform.translation.y -= BLOCK_SIZE;
            
            if !is_valid_position(&new_transform, &game_grid) {
                commands.entity(entity)
                    .remove::<Falling>()
                    .insert(Stopped);
            } else {
                *transform = new_transform;
            }
        }
    }
}

pub fn get_tetromino_block_positions(parent_transform: &Transform) -> Vec<Vec3> {
    let mut positions = Vec::new();
    for i in 0..4 {
        // Create positions: -1.5, -0.5, 0.5, 1.5
        let offset = (i as f32 - 1.5) * BLOCK_SIZE;
        let local_pos = Vec3::new(offset, 0.0, 0.0);
        let world_pos = parent_transform.transform_point(local_pos);
        positions.push(world_pos);
    }
    positions
}

pub fn is_valid_position(transform: &Transform, game_grid: &GameGrid) -> bool {
    println!("\nParent transform position: {:?}", transform.translation);
    
    for (i, block_pos) in get_tetromino_block_positions(transform).iter().enumerate() {
        let (grid_x, grid_y, grid_z) = get_grid_position(*block_pos);

        println!(
            "Block {}: World pos: ({:.1}, {:.1}, {:.1}), Grid pos: ({}, {}, {})",
            i, block_pos.x, block_pos.y, block_pos.z, grid_x, grid_y, grid_z
        );

        // Adjust boundary check
        if grid_x < 0 || grid_x >= GRID_WIDTH as i32 || 
           grid_y <= 0 || grid_y >= GRID_HEIGHT as i32 ||
           grid_z < 0 || grid_z >= GRID_DEPTH as i32 {
            println!("Out of bounds at grid coordinates: ({}, {}, {})", grid_x, grid_y, grid_z);
            return false;
        }

        if !game_grid.is_cell_empty(grid_x, grid_y, grid_z) {
            println!("Collision at grid coordinates: ({}, {}, {})", grid_x, grid_y, grid_z);
            return false;
        }
    }
    true
}

pub fn get_grid_position(world_pos: Vec3) -> (i32, i32, i32) {
    // Convert world coordinates to grid coordinates
    // Add GRID_WIDTH/2 to center the grid around world origin
    let grid_x = (world_pos.x + (GRID_WIDTH as f32 / 2.0)).round() as i32;
    let grid_y = world_pos.y.round() as i32;
    let grid_z = (world_pos.z + (GRID_DEPTH as f32 / 2.0)).round() as i32;
    (grid_x, grid_y, grid_z)
}