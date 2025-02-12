use bevy::prelude::*;
use tetris::game::game_state::{GameGrid, Tetromino, BLOCK_SIZE};
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
        let child_entity = app
            .world_mut()
            .spawn((Transform::from_xyz(2.0, 2.0, 2.0),))
            .id();

        app.world()
            .entity_mut(parent_entity)
            .add_children(&[child_entity]);

        app.insert_resource(GameGrid::new());

        let transform_query = app
            .world()
            .query_filtered::<&Transform, Without<Tetromino>>();

        let children = app.world().get::<Children>(parent_entity).unwrap();

        // let transform = app.world().get::<Transform>(parent_entity).unwrap();
        // let result = is_valid_position(
        //     transform,
        //     &game_grid,
        //     &children,
        //     &transform_query,
        // );

        // assert!(result);
    }
}
