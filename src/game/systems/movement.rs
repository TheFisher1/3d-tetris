use crate::game::game_state::{
    Active, Falling, GameGrid, Stopped, Tetromino, TetrominoBlock, BLOCK_SIZE, FALL_TIME,
    GRID_DEPTH, GRID_HEIGHT, GRID_WIDTH,
};
use crate::game::systems::RowCleaned;
use bevy::prelude::*;

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

        if !is_valid_position_tetromino(&new_transform, &game_grid, children, &transform_query) {
            commands
                .entity(entity)
                .remove::<Falling>()
                .remove::<Active>();

            for &child in children.iter() {
                if let Ok(child_transform) = transform_query.get(child) {
                    let world_pos = transform.transform_point(child_transform.translation);

                    commands
                        .entity(child)
                        .remove_parent()
                        .insert(Transform::from_translation(world_pos))
                        .insert(Stopped)
                        .insert(TetrominoBlock { id: tetromino.id });
                }
            }

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
    mut query: Query<(Entity, &mut Transform, &mut Falling), (With<TetrominoBlock>)>,
) {
    for (entity, mut transform, mut falling) in query.iter_mut() {
        falling.timer.tick(time.delta());
        if !falling.timer.just_finished() {
            continue;
        }

        let mut new_transform = transform.clone();
        new_transform.translation.y -= BLOCK_SIZE;

        let (grid_x, grid_y, grid_z) = get_grid_position(new_transform.translation);

        if grid_y <= 1 || !game_grid.is_cell_empty(grid_x, grid_y - 1, grid_z) {
            commands.entity(entity).remove::<Falling>().insert(Stopped);
        } else {
            *transform = new_transform;
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
                commands
                    .entity(tetromino)
                    .remove::<Stopped>()
                    .insert(Falling {
                        timer: Timer::from_seconds(FALL_TIME, TimerMode::Repeating),
                    });
            }
        }
    }
}

pub fn handle_despawn_event_blocks(
    mut commands: Commands,
    mut event_reader: EventReader<RowCleaned>,
    mut query: Query<(Entity, &mut Transform), (With<TetrominoBlock>, With<Stopped>)>,
    mut game_grid: ResMut<GameGrid>,
) {
    for event in event_reader.read() {
        let row_number = event.0;

        for (entity, mut transform) in query.iter_mut() {
            let current_y = transform.translation.y;

            if current_y > row_number as f32 {
                let (grid_x, grid_y, grid_z) = get_grid_position(transform.translation);
                game_grid.set_cell(grid_x, grid_y, grid_z, -1);

                transform.translation.y -= BLOCK_SIZE;

                let (grid_x, grid_y, grid_z) = get_grid_position(transform.translation);

                if grid_y > 0 && game_grid.is_cell_empty(grid_x, grid_y, grid_z) {
                    commands.entity(entity).remove::<Stopped>().insert(Falling {
                        timer: Timer::from_seconds(FALL_TIME, TimerMode::Repeating),
                    });
                }
            }
        }
    }
}

pub fn is_valid_position(world_position: &Vec3, game_grid: &GameGrid) -> bool {
    let [grid_x, grid_y, grid_z] = world_position.to_array().map(|x| x.round() as i32);
    if grid_x < 1
        || grid_x >= GRID_WIDTH as i32
        || grid_y < 1
        || grid_y >= GRID_HEIGHT as i32
        || grid_z < 1
        || grid_z >= GRID_DEPTH as i32
    {
        return false;
    }

    game_grid.is_cell_empty(grid_x, grid_y, grid_z)
}

pub fn is_valid_position_tetromino(
    transform: &Transform,
    game_grid: &GameGrid,
    children: &Children,
    transform_query: &Query<&Transform, Without<Tetromino>>,
) -> bool {
    children.iter().all(|&child| {
        if let Ok(child_transform) = transform_query.get(child) {
            let world_pos = transform.transform_point(child_transform.translation);
            println!("{:?} -> {:?}, world_pos: {:?}", child, child_transform, world_pos);
            is_valid_position(&world_pos, &game_grid)
        } else {
            false
        }
    })
}

pub fn get_grid_position(world_pos: Vec3) -> (i32, i32, i32) {
    let grid_x = world_pos.x.round() as i32;
    let grid_y = world_pos.y.round() as i32;
    let grid_z = world_pos.z.round() as i32;
    (grid_x, grid_y, grid_z)
}
