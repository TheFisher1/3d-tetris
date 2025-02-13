use crate::game::game_elements::*;
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::*;

pub fn spawn_new_tetromino(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    falling_query: Query<&Tetromino, With<Falling>>,
) {
    let position = Vec3::new(
        GRID_WIDTH as f32,
        GRID_HEIGHT as f32 - 2.0,
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

    let block_mesh = meshes.add(Mesh::from(Cuboid::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE))); // { size: BLOCK_SIZE }));
    let block_material = materials.add(StandardMaterial {
        base_color: COLORS[1].clone(),
        ..default()
    });

    commands
        .spawn((
            Transform::from_xyz(
                adjusted_position.x,
                adjusted_position.y,
                adjusted_position.z,
            ),
            Tetromino { id: 1 },
            Active,
            Falling {
                timer: Timer::from_seconds(FALL_TIME, TimerMode::Repeating),
            },
            MovementCooldown {
                timer: Timer::from_seconds(0.1, TimerMode::Once),
            },
            Visibility::default(),
        ))
        .with_children(|parent| {
            for i in 0..4 {
                parent.spawn((
                    Mesh3d::from(block_mesh.clone()),
                    MeshMaterial3d::from(block_material.clone()),
                    Transform::from_translation(TETROMINO_OFFSETS[1][i]),
                ));
            }
        });

    // parent.spawn(PbrBundle {
    //     mesh: Mesh3d::from(block_mesh.clone()),
    //     material: MeshMaterial3d::from(block_material.clone()),
    //     transform: Transform::from_translation(TETROMINO_OFFSETS[1][i]),//Transform::from_xyz(i as f32, 0.0, 0.0),
    //     ..default()
    // });
}
