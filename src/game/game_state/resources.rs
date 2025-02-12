use crate::game::game_state::constants::{GRID_DEPTH, GRID_HEIGHT, GRID_WIDTH};
use bevy::prelude::*;
//grid - int[][][], -1 - not visited, else - color = Colors[i],

#[derive(Resource)]
pub struct GameGrid {
    cells: Vec<Vec<Vec<isize>>>,
}

impl GameGrid {
    pub fn new() -> Self {
        let cells =
            vec![vec![vec![-1; GRID_DEPTH as usize]; GRID_HEIGHT as usize]; GRID_WIDTH as usize];
        Self { cells }
    }

    pub fn is_cell_empty(&self, x: i32, y: i32, z: i32) -> bool {
        if x < 0 || y < 0 || z < 0 {
            return false;
        }

        let x = x as usize;
        let y = y as usize;
        let z = z as usize;

        if x >= GRID_WIDTH as usize || y >= GRID_HEIGHT as usize || z >= GRID_DEPTH as usize {
            return false;
        }

        self.cells[x][y][z] == -1
    }

    pub fn set_cell(&mut self, x: i32, y: i32, z: i32, occupied: isize) {
        if x >= 0 && y >= 0 && z >= 0 {
            let x = x as usize;
            let y = y as usize;
            let z = z as usize;

            if x < GRID_WIDTH as usize && y < GRID_HEIGHT as usize && z < GRID_DEPTH as usize {
                self.cells[x][y][z] = occupied;
            }
        }
    }

    pub fn is_row_occupied(&self, y: u32) -> bool {
        if y == 0 {
            return true;
        }

        if y == GRID_HEIGHT {
            return true;
        }

        (1..GRID_DEPTH)
            .all(|x| (1..GRID_WIDTH).all(|z| !self.is_cell_empty(x as i32, y as i32, z as i32)))
    }

    pub fn clear_row(&mut self, y: u32) {
        if y == 0 || y == GRID_HEIGHT {
            return;
        }

        (1..GRID_DEPTH).for_each(|x| {
            (1..GRID_WIDTH).for_each(|z| {
                self.set_cell(x as i32, y as i32, z as i32, -1);
            })
        })
    }
}
