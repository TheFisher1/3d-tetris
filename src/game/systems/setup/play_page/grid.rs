use crate::game::game_elements::{BLOCK_SIZE, GRID_DEPTH, GRID_HEIGHT, GRID_WIDTH};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{
    default, AlphaMode, Commands, Component, Cuboid, Mesh, Mesh3d, ResMut, Transform,
};

#[derive(Component)]
pub struct Grid;

pub fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let wall_mesh = meshes.add(Mesh::from(Cuboid::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE)));
    let wall_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.5, 0.5, 0.5, 0.2),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    for x in 0..=GRID_WIDTH {
        for z in 0..=GRID_DEPTH {
            commands.spawn((
                Mesh3d::from(wall_mesh.clone()),
                MeshMaterial3d::from(wall_material.clone()),
                Transform::from_xyz(x as f32 * BLOCK_SIZE, 0.0, z as f32 * BLOCK_SIZE),
                Grid,
            ));
        }
    }

    for x in [0, GRID_WIDTH] {
        for z in [0, GRID_DEPTH] {
            for y in 0..=GRID_HEIGHT {
                commands.spawn((
                    Mesh3d::from(wall_mesh.clone()),
                    MeshMaterial3d::from(wall_material.clone()),
                    Transform::from_xyz(
                        x as f32 * BLOCK_SIZE,
                        y as f32 * BLOCK_SIZE,
                        z as f32 * BLOCK_SIZE,
                    ),
                    Grid,
                ));
            }
        }
    }

    let y = GRID_HEIGHT;

    for x in 0..=GRID_WIDTH {
        for z in [0, GRID_DEPTH] {
            commands.spawn((
                Mesh3d::from(wall_mesh.clone()),
                MeshMaterial3d::from(wall_material.clone()),
                Transform::from_xyz(
                    x as f32 * BLOCK_SIZE,
                    y as f32 * BLOCK_SIZE,
                    z as f32 * BLOCK_SIZE,
                ),
                Grid,
            ));
        }
    }

    for z in 0..=GRID_DEPTH {
        for x in [0, GRID_WIDTH] {
            commands.spawn((
                Mesh3d::from(wall_mesh.clone()),
                MeshMaterial3d::from(wall_material.clone()),
                Transform::from_xyz(
                    x as f32 * BLOCK_SIZE,
                    y as f32 * BLOCK_SIZE,
                    z as f32 * BLOCK_SIZE,
                ),
                Grid,
            ));
        }
    }
}
