use bevy::prelude::*;

#[cfg(test)]
mod tests {
    use example_repo::{cleanup, GameGrid, RowCleaned, GRID_DEPTH, GRID_WIDTH};
    use super::*;

    #[test]
    fn test_row_clearing() {
        let mut app = App::new();

        app.add_event::<RowCleaned>();
        
        app.add_systems(Update, cleanup);
        
        let mut game_grid = GameGrid::new();
        
        let test_row: u32 = 5;
        for x in 1..GRID_DEPTH {
            for z in 1..GRID_WIDTH {
                game_grid.set_cell(x as i32, test_row as i32, z as i32, 1);
            }
        }
        
        app.insert_resource(game_grid);

        app.update();
        
        let row_cleaned_events = app.world().resource::<Events<RowCleaned>>();
        let mut reader = row_cleaned_events.get_reader();
        assert_eq!(reader.read(row_cleaned_events).count(), 1);

        let mut game_grid = app.world_mut().resource_mut::<GameGrid>();
        game_grid.set_cell(1, test_row as i32, 1, 1);

        app.update();

        let row_cleaned_events = app.world().resource::<Events<RowCleaned>>();
        let mut reader = row_cleaned_events.get_reader();
        let events: Vec<_> = reader.read(row_cleaned_events).collect();
        
        assert_eq!(events.len(), 1);
        
        assert_eq!(events[0].0, test_row);

        let game_grid = app.world().resource::<GameGrid>();
        assert_eq!(false, game_grid.is_row_occupied(test_row));
    }

    #[test]
    fn test_multiple_rows_clearing() {
        let mut app = App::new();
        app.add_event::<RowCleaned>();
        app.add_systems(Update, cleanup);
        
        let mut game_grid = GameGrid::new();
        
        let test_rows: [u32; 2] = [5, 6];
        for row in test_rows.iter() {
            for x in 1..GRID_WIDTH {
                for z in 1..GRID_DEPTH {
                    game_grid.set_cell(x as i32, *row as i32, z as i32, 1);
                }
            }
        }
        
        app.insert_resource(game_grid);
        app.update();

        let row_cleaned_events = app.world().resource::<Events<RowCleaned>>();
        let mut reader = row_cleaned_events.get_reader();
        let events: Vec<_> = reader.read(row_cleaned_events).collect();
        
        assert_eq!(events.len(), 2);
        assert!(events.iter().any(|e| e.0 == test_rows[0]));
        assert!(events.iter().any(|e| e.0 == test_rows[1]));
    }
} 