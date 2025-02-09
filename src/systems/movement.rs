use bevy::prelude::*;
use crate::game_state::{Active, Falling, GameGrid, MovementCooldown, Stopped, Tetromino, BLOCK_SIZE, GRID_DEPTH, GRID_HEIGHT, GRID_WIDTH, ROTATION_DEGREES};
use crate::systems::{RowCleaned};
use crate::game_state::constants::FALL_TIME;

pub fn keyboard_system(
    time: Res<Time>,
    game_grid: Res<GameGrid>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut MovementCooldown), (With<Tetromino>, With<Falling>, With<Active>)>
) {
    let (mut transform, mut cooldown) = query.single_mut();

    if keyboard.just_pressed(KeyCode::Space) {
        let mut test_transform = transform.clone();
        let mut highest_valid_y = test_transform.translation.y;

        let mut highest_valid_y_reached = true;
        while test_transform.translation.y > 0.0 && highest_valid_y_reached {
            test_transform.translation.y -= BLOCK_SIZE;
            if !is_valid_position(&test_transform, &game_grid) {
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
                    .remove::<Active>()
                    .insert(Stopped);
            } else {

                *transform = new_transform;
            }
        }
    }
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

pub fn get_tetromino_block_positions(parent_transform: &Transform) -> Vec<Vec3> {
    let mut positions = Vec::new();
    for i in 0..4 {
        let offset = (i as f32) * BLOCK_SIZE;
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
        if grid_x <= 0 || grid_x >= GRID_WIDTH as i32 ||
           grid_y <= 0 || grid_y >= GRID_HEIGHT as i32 ||
           grid_z <= 0 || grid_z >= GRID_DEPTH as i32 {
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
    let grid_x = world_pos.x.round() as i32;
    let grid_y = world_pos.y.round() as i32;
    let grid_z = world_pos.z.round() as i32;
    (grid_x, grid_y, grid_z)
}