use bevy::prelude::*;

// Constants for the game
const BLOCK_SIZE: f32 = 1.0;
const FALL_TIME: f32 = 1.0; // Time in seconds between falls
const GRID_WIDTH: u32 = 10;
const GRID_HEIGHT: u32 = 20;
const GRID_DEPTH: u32 = 10;
const CAMERA_RADIUS: f32 = 25.0;
const CAMERA_HEIGHT: f32 = 20.0;
const ROTATION_SPEED: f32 = 1.0;
const MOVE_SPEED: f32 = 5.0; // Units per second (reduced from instant movement)
const ROTATION_DEGREES: f32 = 90.0; // Rotate by 90 degrees each time
const EMPTY_CELL: i32 = 0;
const FILLED_CELL: i32 = 1;

// Component to mark a tetromino
#[derive(Component)]
struct Tetromino;

// Component for falling behavior
#[derive(Component)]
struct Falling {
    timer: Timer,
}

#[derive(Component)]
struct GridWall;

// Add this component to mark blocks that have stopped falling
#[derive(Component)]
struct Stopped;

// Add this component for the camera
#[derive(Component)]
struct GameCamera {
    angle: f32,
    height: f32,
}

#[derive(Clone, Copy)]
struct GridPosition {
    x: i32,
    y: i32,
    z: i32,
}

// Add this component for movement cooldown
#[derive(Component)]
struct MovementCooldown {
    timer: Timer,
}

// Add this resource to track the grid state
#[derive(Resource)]
struct GameGrid {
    cells: Vec<Vec<Vec<i32>>>,
}

impl GameGrid {
    fn new() -> Self {
        let cells = vec![vec![vec![EMPTY_CELL; GRID_DEPTH as usize]; GRID_HEIGHT as usize]; GRID_WIDTH as usize];
        Self { cells }
    }

    fn is_cell_empty(&self, x: i32, y: i32, z: i32) -> bool {
        if x < 0 || x >= GRID_WIDTH as i32 || 
           y < 0 || y >= GRID_HEIGHT as i32 ||
           z < 0 || z >= GRID_DEPTH as i32 {
            return false;
        }
        self.cells[x as usize][y as usize][z as usize] == EMPTY_CELL
    }

    fn set_cell(&mut self, x: i32, y: i32, z: i32, value: i32) {
        if x >= 0 && x < GRID_WIDTH as i32 &&
           y >= 0 && y < GRID_HEIGHT as i32 &&
           z >= 0 && z < GRID_DEPTH as i32 {
            self.cells[x as usize][y as usize][z as usize] = value;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameGrid::new())
        .add_systems(Startup, (setup, spawn_grid))
        .add_systems(Update, (
            falling,
            rotate_camera,
            move_tetromino,
            update_grid_state,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-15.0, CAMERA_HEIGHT, -15.0)
                .looking_at(Vec3::new(0.0, 5.0, 0.0), Vec3::Y),
            ..default()
        },
        GameCamera { 
            angle: 0.0,
            height: CAMERA_HEIGHT,
        },
    ));

    Vec3::new(1.0f32, 1.0f32, 1.0f32).to_array();

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Spawn a simple I-tetromino
    let block_mesh = meshes.add(Mesh::from(shape::Cube { size: BLOCK_SIZE }));
    let block_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 0.8, 0.8),
        ..default()
    });

    // Update initial position to be at the center top of the grid
    let initial_x = 0.0;  // Center X (will be offset by child blocks)
    let initial_y = (GRID_HEIGHT as f32) - 2.0;  // Near top
    let initial_z = 0.0;  // Center Z

    commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(initial_x, initial_y, initial_z)),
            Tetromino,
            Falling {
                timer: Timer::from_seconds(FALL_TIME, TimerMode::Repeating),
            },
            MovementCooldown {
                timer: Timer::from_seconds(0.1, TimerMode::Once),
            },
        ))
        .with_children(|parent| {
            for i in 0..4 {
                parent.spawn(PbrBundle {
                    mesh: block_mesh.clone(),
                    material: block_material.clone(),
                    transform: Transform::from_xyz(i as f32 * BLOCK_SIZE - 1.0, 0.0, 0.0),
                    ..default()
                });
            }
        });
}

fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let wall_mesh = meshes.add(Mesh::from(shape::Cube { size: BLOCK_SIZE }));
    let wall_material = materials.add(StandardMaterial {
        base_color: Color::rgba(0.5, 0.5, 0.5, 0.2),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // Spawn ground plane with transparent blocks
    for x in 0..=GRID_WIDTH {
        for z in 0..=GRID_DEPTH {
            commands.spawn((
                PbrBundle {
                    mesh: wall_mesh.clone(),
                    material: wall_material.clone(),
                    transform: Transform::from_xyz(
                        x as f32 * BLOCK_SIZE - GRID_WIDTH as f32 / 2.0,
                        0.0,
                        z as f32 * BLOCK_SIZE - GRID_DEPTH as f32 / 2.0,
                    ),
                    ..default()
                },
                GridWall,
            ));
        }
    }

    // Spawn vertical edges (4 corners)
    for x in [0, GRID_WIDTH] {
        for z in [0, GRID_DEPTH] {
            for y in 0..=GRID_HEIGHT {
                commands.spawn((
                    PbrBundle {
                        mesh: wall_mesh.clone(),
                        material: wall_material.clone(),
                        transform: Transform::from_xyz(
                            x as f32 * BLOCK_SIZE - GRID_WIDTH as f32 / 2.0,
                            y as f32 * BLOCK_SIZE,
                            z as f32 * BLOCK_SIZE - GRID_DEPTH as f32 / 2.0,
                        ),
                        ..default()
                    },
                    GridWall,
                ));
            }
        }
    }

    // Spawn top frame only (we already have ground plane)
    let y = GRID_HEIGHT;
    // Spawn X-axis edges
    for x in 0..=GRID_WIDTH {
        for z in [0, GRID_DEPTH] {
            commands.spawn((
                PbrBundle {
                    mesh: wall_mesh.clone(),
                    material: wall_material.clone(),
                    transform: Transform::from_xyz(
                        x as f32 * BLOCK_SIZE - GRID_WIDTH as f32 / 2.0,
                        y as f32 * BLOCK_SIZE,
                        z as f32 * BLOCK_SIZE - GRID_DEPTH as f32 / 2.0,
                    ),
                    ..default()
                },
                GridWall,
            ));
        }
    }

    // Spawn Z-axis edges for top
    for z in 0..=GRID_DEPTH {
        for x in [0, GRID_WIDTH] {
            commands.spawn((
                PbrBundle {
                    mesh: wall_mesh.clone(),
                    material: wall_material.clone(),
                    transform: Transform::from_xyz(
                        x as f32 * BLOCK_SIZE - GRID_WIDTH as f32 / 2.0,
                        y as f32 * BLOCK_SIZE,
                        z as f32 * BLOCK_SIZE - GRID_DEPTH as f32 / 2.0,
                    ),
                    ..default()
                },
                GridWall,
            ));
        }
    }
}

fn falling(
    time: Res<Time>,
    game_grid: Res<GameGrid>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Falling), With<Tetromino>>,
) {
    for (entity, mut transform, mut falling) in query.iter_mut() {
        falling.timer.tick(time.delta());

        if falling.timer.just_finished() {
            let mut new_transform = transform.clone();
            new_transform.translation.y -= BLOCK_SIZE;
            
            if !is_valid_position(&new_transform, &game_grid) {
                commands.entity(entity)
                    .remove::<Falling>()
                    .insert(Stopped);
            } else {
                *transform = new_transform;
            }
        }
    }
}

// Update the camera rotation system
fn rotate_camera(
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

// Update the movement system to include rotation
fn move_tetromino(
    time: Res<Time>,
    game_grid: Res<GameGrid>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut MovementCooldown), With<Tetromino>>,
) {
    let (mut transform, mut cooldown) = query.single_mut();
    cooldown.timer.tick(time.delta());

    if cooldown.timer.finished() {
        let mut moved = false;
        let mut new_transform = transform.clone();

        // Movement controls - move by exactly one block
        if keyboard.just_pressed(KeyCode::A) {
            new_transform.translation.x -= BLOCK_SIZE;
            moved = true;
        }
        if keyboard.just_pressed(KeyCode::D) {
            new_transform.translation.x += BLOCK_SIZE;
            moved = true;
        }
        if keyboard.just_pressed(KeyCode::W) {
            new_transform.translation.z -= BLOCK_SIZE;
            moved = true;
        }
        if keyboard.just_pressed(KeyCode::S) {
            new_transform.translation.z += BLOCK_SIZE;
            moved = true;
        }

        // Rotation controls
        if keyboard.just_pressed(KeyCode::Q) {
            new_transform.rotate_y(ROTATION_DEGREES.to_radians());
            // After rotation, ensure blocks are grid-aligned
            new_transform.translation = new_transform.translation.round();
            moved = true;
        }
        if keyboard.just_pressed(KeyCode::E) {
            new_transform.rotate_y(-ROTATION_DEGREES.to_radians());
            // After rotation, ensure blocks are grid-aligned
            new_transform.translation = new_transform.translation.round();
            moved = true;
        }

        if moved {
            if is_valid_position(&new_transform, &game_grid) {
                *transform = new_transform;
            }
            cooldown.timer.reset();
        }
    }
}

// Update the block position calculation to account for rotation
fn get_tetromino_block_positions(parent_transform: &Transform) -> Vec<Vec3> {
    let mut positions = Vec::new();
    for i in 0..4 {
        // Create positions: -1.5, -0.5, 0.5, 1.5
        let offset = (i as f32 - 1.5) * BLOCK_SIZE;
        let local_pos = Vec3::new(offset, 0.0, 0.0);
        let world_pos = parent_transform.transform_point(local_pos);
        positions.push(world_pos);
    }
    positions
}

// Update the coordinate conversion in is_valid_position and update_grid_state
fn get_grid_position(world_pos: Vec3) -> (i32, i32, i32) {
    // Convert world coordinates to grid coordinates
    // Add GRID_WIDTH/2 to center the grid around world origin
    let grid_x = (world_pos.x + (GRID_WIDTH as f32 / 2.0)).round() as i32;
    let grid_y = world_pos.y.round() as i32;
    let grid_z = (world_pos.z + (GRID_DEPTH as f32 / 2.0)).round() as i32;
    (grid_x, grid_y, grid_z)
}

fn is_valid_position(transform: &Transform, game_grid: &GameGrid) -> bool {
    println!("\nParent transform position: {:?}", transform.translation);
    
    for (i, block_pos) in get_tetromino_block_positions(transform).iter().enumerate() {
        let (grid_x, grid_y, grid_z) = get_grid_position(*block_pos);

        println!(
            "Block {}: World pos: ({:.1}, {:.1}, {:.1}), Grid pos: ({}, {}, {})",
            i, block_pos.x, block_pos.y, block_pos.z, grid_x, grid_y, grid_z
        );

        // Adjust boundary check
        if grid_x < 0 || grid_x >= GRID_WIDTH as i32 || 
           grid_y <= 0 || grid_y >= GRID_HEIGHT as i32 ||
           grid_z < 0 || grid_z >= GRID_DEPTH as i32 {
            println!("Out of bounds at grid coordinates: ({}, {}, {})", grid_x, grid_y, grid_z);
            return false;
        }

        if !game_grid.is_cell_empty(grid_x, grid_y, grid_z) {
            println!("Collision at grid coordinates: ({}, {}, {})", grid_x, grid_y, grid_z);
            return false;
        }
    }
    true
}

// Add new system to update grid state when tetrominoes stop
fn update_grid_state(
    mut game_grid: ResMut<GameGrid>,
    query: Query<&Transform, Added<Stopped>>,
) {
    for transform in query.iter() {
        println!("\nPlacing stopped blocks at:");
        for block_pos in get_tetromino_block_positions(transform) {
            let (grid_x, grid_y, grid_z) = get_grid_position(block_pos);
            
            println!(
                "World pos: ({:.1}, {:.1}, {:.1}), Grid pos: ({}, {}, {})",
                block_pos.x, block_pos.y, block_pos.z, grid_x, grid_y, grid_z
            );
            
            game_grid.set_cell(grid_x, grid_y, grid_z, FILLED_CELL);
        }
    }
} 