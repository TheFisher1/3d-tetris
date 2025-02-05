use bevy::prelude::*;
use crate::components::GameCamera;
use crate::constants::*;

pub fn rotate_camera(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut GameCamera)>,
) {
    let (mut transform, mut camera) = query.single_mut();
    let dt = time.delta_seconds();
    
    // Horizontal rotation
    if keyboard.pressed(KeyCode::Left) {
        camera.angle += ROTATION_SPEED * dt;
    }
    if keyboard.pressed(KeyCode::Right) {
        camera.angle -= ROTATION_SPEED * dt;
    }

    // Vertical movement
    if keyboard.pressed(KeyCode::Up) {
        camera.height += ROTATION_SPEED * 10.0 * dt;
        camera.height = camera.height.min(50.0);  // Maximum height
    }
    if keyboard.pressed(KeyCode::Down) {
        camera.height -= ROTATION_SPEED * 10.0 * dt;
        camera.height = camera.height.max(5.0);   // Minimum height
    }

    // Update camera position
    let x = camera.angle.cos() * CAMERA_RADIUS;
    let z = camera.angle.sin() * CAMERA_RADIUS;
    
    transform.translation = Vec3::new(x, camera.height, z);
    transform.look_at(Vec3::new(0.0, 5.0, 0.0), Vec3::Y);
}