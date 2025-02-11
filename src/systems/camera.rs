use bevy::prelude::*;
use crate::game_state::{GameCamera, CAMERA_RADIUS, ROTATION_SPEED};

pub fn rotate_camera(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut GameCamera)>,
) {
    if let Ok((mut transform, mut camera)) = query.get_single_mut() {
        let dt = time.delta().as_secs_f32();

        if keyboard.pressed(KeyCode::ArrowLeft) {
            camera.angle += ROTATION_SPEED * dt;
        }

        if keyboard.pressed(KeyCode::ArrowRight) {
            camera.angle -= ROTATION_SPEED * dt;
        }

        if keyboard.pressed(KeyCode::ArrowUp) {
            camera.height += ROTATION_SPEED * 10.0 * dt;
            camera.height = camera.height.min(50.0);  // Maximum height
        }

        if keyboard.pressed(KeyCode::ArrowDown) {
            camera.height -= ROTATION_SPEED * 10.0 * dt;
            camera.height = camera.height.max(5.0);   // Minimum height
        }

        let x = camera.angle.cos() * CAMERA_RADIUS;
        let z = camera.angle.sin() * CAMERA_RADIUS;

        transform.translation = Vec3::new(x, camera.height, z);
        transform.look_at(Vec3::new(0.0, 5.0, 0.0), Vec3::Y);
    }
}