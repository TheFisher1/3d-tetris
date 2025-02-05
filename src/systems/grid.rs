use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let wall_mesh = meshes.add(Mesh::from(shape::Cube { size: BLOCK_SIZE }));
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
                    mesh: wall_mesh.clone(),
                    material: wall_material.clone(),
                    transform: Transform::from_xyz(
                        x as f32 * BLOCK_SIZE - GRID_WIDTH as f32 / 2.0,
                        0.0,
                        z as f32 * BLOCK_SIZE - GRID_DEPTH as f32 / 2.0,
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
                        mesh: wall_mesh.clone(),
                        material: wall_material.clone(),
                        transform: Transform::from_xyz(
                            x as f32 * BLOCK_SIZE - GRID_WIDTH as f32 / 2.0,
                            y as f32 * BLOCK_SIZE,
                            z as f32 * BLOCK_SIZE - GRID_DEPTH as f32 / 2.0,
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
                    mesh: wall_mesh.clone(),
                    material: wall_material.clone(),
                    transform: Transform::from_xyz(
                        x as f32 * BLOCK_SIZE - GRID_WIDTH as f32 / 2.0,
                        y as f32 * BLOCK_SIZE,
                        z as f32 * BLOCK_SIZE - GRID_DEPTH as f32 / 2.0,
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
                    mesh: wall_mesh.clone(),
                    material: wall_material.clone(),
                    transform: Transform::from_xyz(
                        x as f32 * BLOCK_SIZE - GRID_WIDTH as f32 / 2.0,
                        y as f32 * BLOCK_SIZE,
                        z as f32 * BLOCK_SIZE - GRID_DEPTH as f32 / 2.0,
                    ),
                    ..default()
                },
                GridWall,
            ));
        }
    }
}