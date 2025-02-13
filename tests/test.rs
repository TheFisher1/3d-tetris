use bevy::prelude::*;
use tetris::game::game_elements::GameGrid;
use tetris::game::systems::movement::is_valid_position;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_position() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        let game_grid = GameGrid::new();

        let parent_entity = app
            .world_mut()
            .spawn((Transform::from_xyz(1.0, 1.0, 1.0),))
            .id();

        app.insert_resource(GameGrid::new());

        let transform = app.world().get::<Transform>(parent_entity).unwrap();

        assert!(is_valid_position(
            &(transform.rotation * transform.translation),
            &game_grid,
        ));

        let new_parent_entity = app
            .world_mut()
            .spawn((Transform::from_xyz(50.0, 1.0, 1.0),))
            .id();

        let transform = app.world().get::<Transform>(new_parent_entity).unwrap();
        assert!(!is_valid_position(
            &(transform.rotation * transform.translation),
            &game_grid
        ));
    }
}
