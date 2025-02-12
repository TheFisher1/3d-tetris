use crate::game::game_state::{GridWall, BLOCK_SIZE, GRID_DEPTH, GRID_HEIGHT, GRID_WIDTH};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::pbr::{MeshMaterial3d, PbrBundle, StandardMaterial};
use bevy::prelude::{default, AlphaMode, Commands, Cuboid, Mesh, Mesh3d, ResMut, Transform};

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
