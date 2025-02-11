use bevy::prelude::*;

#[cfg(test)]
mod tests {
    use example_repo::{GameGrid, Tetromino, BLOCK_SIZE};
    use example_repo::systems::{is_valid_position, to_tuple};
    use super::*;

    #[test]
    fn test_grid_position_conversion() {
        let world_pos = Vec3::new(2.0, 3.0, 1.0);
        let (x, y, z) = to_tuple(world_pos);
        assert_eq!(x, 2);
        assert_eq!(y, 3);
        assert_eq!(z, 1);
    }

    #[test]
    fn test_valid_position() {
        let game_grid = GameGrid::new();

        let tetromino = Tetromino {
            id: 1,
            offsets: vec![
                Vec3::ZERO,
                Vec3::new(BLOCK_SIZE, 0.0, 0.0),
                Vec3::new(2.0 * BLOCK_SIZE, 0.0, 0.0),
                Vec3::new(3.0 * BLOCK_SIZE, 0.0, 0.0),
            ],
        };

        let valid_transform = Transform::from_xyz(5.0, 5.0, 5.0);
        assert!(is_valid_position(&valid_transform, &game_grid, &tetromino));

        let invalid_transform = Transform::from_xyz(-1.0, 5.0, 5.0);
        assert!(!is_valid_position(&invalid_transform, &game_grid, &tetromino));
    }
}