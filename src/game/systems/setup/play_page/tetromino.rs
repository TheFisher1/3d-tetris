use crate::game::game_elements::*;
use crate::game::state::game_info::{GameInfo, GameState};
use crate::game::systems::is_valid_position_tetromino;
use crate::game::systems::Lock;
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::*;

pub fn spawn_new_tetromino(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_info: ResMut<GameInfo>,
    falling_query: Query<&Tetromino, (With<Falling>, With<Active>)>,
) {
    let position = Vec3::new(
        GRID_WIDTH as f32,
        GRID_HEIGHT as f32 - 4.0,
        GRID_DEPTH as f32,
    );

    if !falling_query.is_empty() {
        return;
    }

    let adjusted_position = Vec3::new(
        position.x - (GRID_WIDTH as f32 / 2.0),
        position.y,
        position.z - (GRID_DEPTH as f32 / 2.0),
    );

    let color_index = game_info.curr_tetromino.id as usize % COLORS.len();
    let block_mesh = meshes.add(Mesh::from(Cuboid::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE)));
    let block_material = materials.add(StandardMaterial {
        base_color: COLORS[color_index].clone(),
        ..default()
    });

    let tetromino_to_spawn = game_info.curr_tetromino.clone();

    commands
        .spawn((
            Transform::from_xyz(
                adjusted_position.x,
                adjusted_position.y,
                adjusted_position.z,
            ),
            tetromino_to_spawn,
            Active,
            JustSpawned,
            Falling {
                timer: Timer::from_seconds(FALL_TIME, TimerMode::Repeating),
            },
            MovementCooldown {
                timer: Timer::from_seconds(0.1, TimerMode::Once),
            },
            Visibility::default(),
        ))
        .with_children(spawn_tetromino_blocks(
            game_info.curr_tetromino.tetromino_type.clone(),
        )(Mesh3d::from(block_mesh), block_material));
}

pub fn check_is_game_over(
    game_grid: Res<GameGrid>,
    query: Query<(&Transform, &Children), (With<JustSpawned>, Without<Lock>)>,
    transform_query: Query<&Transform, Without<Tetromino>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok((transform, children)) = query.get_single() {
        if !is_valid_position_tetromino(transform, &game_grid, children, &transform_query) {
            game_state.set(GameState::StartPage);
        }
    }
}

pub fn spawn_tetromino_blocks<M: Material>(
    tetromino_type: TetrominoType,
) -> Box<dyn Fn(Mesh3d, Handle<M>) -> Box<dyn Fn(&mut ChildBuilder) -> ()>> {
    match tetromino_type {
        TetrominoType::I => gen_spawn_offsets_fn(TETROMINO_TYPE_I_INDEX),
        TetrominoType::L => gen_spawn_offsets_fn(TETROMINO_TYPE_L_INDEX),
        TetrominoType::S => gen_spawn_offsets_fn(TETROMINO_TYPE_S_INDEX),
        TetrominoType::O => gen_spawn_offsets_fn(TETROMINO_TYPE_O_INDEX),
    }
}

fn gen_spawn_offsets_fn<M: Material>(
    tetromino_type_index: usize,
) -> Box<dyn Fn(Mesh3d, Handle<M>) -> Box<dyn Fn(&mut ChildBuilder) -> ()>> {
    Box::new(move |mesh: Mesh3d, material: Handle<M>| {
        Box::new(move |parent: &mut ChildBuilder| {
            for i in 0..TETROMINO_OFFSETS[tetromino_type_index].len() {
                let curr_tetromino_offset = TETROMINO_OFFSETS[tetromino_type_index];
                parent.spawn((
                    Mesh3d::from(mesh.clone()),
                    MeshMaterial3d::from(material.clone()),
                    Transform::from_translation(curr_tetromino_offset[i]),
                ));
            }
        })
    })
}
