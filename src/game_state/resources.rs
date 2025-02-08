use bevy::prelude::*;
use crate::game_state::constants::{GRID_DEPTH, GRID_HEIGHT, GRID_WIDTH};
//grid - int[][][], -1 - not visited, else - color = Colors[i],

#[derive(Resource)]
pub struct GameGrid {
    cells: Vec<Vec<Vec<isize>>>
}

impl GameGrid {
    pub fn new() -> Self {
        let cells = vec![vec![vec![-1; GRID_DEPTH as usize]; GRID_HEIGHT as usize]; GRID_WIDTH as usize];
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
}