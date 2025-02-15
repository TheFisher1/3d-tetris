use tetris::game::game_elements::{Tetromino, TetrominoType, TYPES};

use tetris::game_elements::TETROMINO_OFFSETS;

#[test]
fn test_tetromino_offsets() {
    for offsets in TETROMINO_OFFSETS.iter() {
        assert!(offsets.len() >= 4);

        for offset in offsets.iter() {
            assert!(offset.x >= -3.0 && offset.x <= 3.0);
            assert!(offset.y >= -3.0 && offset.y <= 3.0);
            assert!(offset.z >= -3.0 && offset.z <= 3.0);
        }
    }
}

#[test]
fn test_tetromino_creation() {
    let tetromino = Tetromino::default();
    assert_eq!(tetromino.id, 0);
}

#[test]
fn test_tetromino_iteration() {
    let mut tetromino = Tetromino::default();
    let next = tetromino.next().unwrap();
    assert_eq!(next.id, tetromino.id + 1);
}

#[test]
fn test_tetromino_type_random() {
    let mut types_found = vec![false; TYPES.len()];
    for _ in 0..100 {
        let tetromino = Tetromino::default();
        match tetromino.tetromino_type {
            TetrominoType::I => types_found[0] = true,
            TetrominoType::L => types_found[1] = true,
            TetrominoType::S => types_found[2] = true,
            TetrominoType::O => types_found[3] = true,
        }
    }

    assert!(types_found.iter().all(|&x| x));
}
