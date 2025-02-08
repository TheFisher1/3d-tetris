use bevy::prelude::*;
use crate::game_state::{GameGrid, GridWall, Stopped, Tetromino, BLOCK_SIZE, GRID_DEPTH, GRID_HEIGHT, GRID_WIDTH};
use crate::systems::{get_grid_position, get_tetromino_block_positions};

pub fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let wall_mesh = meshes.add(Mesh::from(Cuboid::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE)));
    let wall_material = materials.add(StandardMaterial {
        base_color: Color::rgba(0.5, 0.5, 0.5, 0.2),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // Spawn ground plane with transparent blocks
    for x in 0..=GRID_WIDTH {
        for z in 0..=GRID_DEPTH {
            commands.spawn((
                PbrBundle {
                    mesh: Mesh3d::from(wall_mesh.clone()),
                    material: MeshMaterial3d::from(wall_material.clone()),
                    transform: Transform::from_xyz(
                        x as f32 * BLOCK_SIZE,
                        0.0,
                        z as f32 * BLOCK_SIZE,
                    ),
                    ..default()
                },
                GridWall,
            ));
        }
    }

    // Spawn vertical edges (4 corners)
    for x in [0, GRID_WIDTH] {
        for z in [0, GRID_DEPTH] {
            for y in 0..=GRID_HEIGHT {
                commands.spawn((
                    PbrBundle {
                        mesh: Mesh3d::from(wall_mesh.clone()),
                        material: MeshMaterial3d::from(wall_material.clone()),
                        transform: Transform::from_xyz(
                            x as f32 * BLOCK_SIZE,
                            y as f32 * BLOCK_SIZE,
                            z as f32 * BLOCK_SIZE,
                        ),
                        ..default()
                    },
                    GridWall,
                ));
            }
        }
    }

    // Spawn top frame only (we already have ground plane)
    let y = GRID_HEIGHT;
    // Spawn X-axis edges
    for x in 0..=GRID_WIDTH {
        for z in [0, GRID_DEPTH] {
            commands.spawn((
                PbrBundle {
                    mesh: Mesh3d::from(wall_mesh.clone()),
                    material: MeshMaterial3d::from(wall_material.clone()),
                    transform: Transform::from_xyz(
                        x as f32 * BLOCK_SIZE,
                        y as f32 * BLOCK_SIZE,
                        z as f32 * BLOCK_SIZE,
                    ),
                    ..default()
                },
                GridWall,
            ));
        }
    }

    // Spawn Z-axis edges for top
    for z in 0..=GRID_DEPTH {
        for x in [0, GRID_WIDTH] {
            commands.spawn((
                PbrBundle {
                    mesh: Mesh3d::from(wall_mesh.clone()),
                    material: MeshMaterial3d::from(wall_material.clone()),
                    transform: Transform::from_xyz(
                        x as f32 * BLOCK_SIZE,
                        y as f32 * BLOCK_SIZE,
                        z as f32 * BLOCK_SIZE,
                    ),
                    ..default()
                },
                GridWall,
            ));
        }
    }
}

pub fn update_grid_state(
    mut game_grid: ResMut<GameGrid>,
    query: Query<(&Transform, &Tetromino), Added<Stopped>>,
) {
    for (transform, tetromino) in query.iter() {
        for block_pos in get_tetromino_block_positions(transform) {
            let (grid_x, grid_y, grid_z) = get_grid_position(block_pos);

            game_grid.set_cell(grid_x, grid_y, grid_z, tetromino.id);
        }
    }
}