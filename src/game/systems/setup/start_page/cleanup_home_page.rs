use bevy::prelude::{Camera2d, Commands, DespawnRecursiveExt, Entity, Query, With};
use crate::game::systems::NavigationGroup;

pub fn cleanup_home_page(mut commands: Commands, query: Query<Entity, With<NavigationGroup>>, camera_query: Query<Entity, With<Camera2d>>) {
    if let Ok(entity) = camera_query.get_single() {
        commands.entity(entity).despawn();
    }

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}