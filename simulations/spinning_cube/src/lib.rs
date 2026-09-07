use bevy::prelude::*;
use wasm_bindgen::prelude::*;

// This annotation exposes the function to JavaScript
#[wasm_bindgen(start)]
pub fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy-canvas".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_ising)
        .add_systems(Update, update_spins)
        .run();
}

#[derive(Component)]
struct Spin(i8); // 1 or -1

#[derive(Component)]
struct GridPos {
    x: usize,
    y: usize,
}

const GRID_SIZE: usize = 20;
const CELL_SIZE: f32 = 15.0;

fn setup_ising(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    let offset = (GRID_SIZE as f32 * CELL_SIZE) / 2.0;

    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let spin_val = if (x + y) % 2 == 0 { 1 } else { -1 };
            let color = if spin_val > 0 {
                Color::WHITE
            } else {
                Color::BLACK
            };

            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(CELL_SIZE - 1.0, CELL_SIZE - 1.0)),
                    ..default()
                },
                Transform::from_xyz(
                    x as f32 * CELL_SIZE - offset,
                    y as f32 * CELL_SIZE - offset,
                    0.0,
                ),
                Spin(spin_val),
                GridPos { x, y },
            ));
        }
    }
}

fn update_spins(mut query: Query<(&mut Spin, &mut Sprite)>, time: Res<Time>) {
    // Simple Mock: Randomly flip spins to simulate "temperature"
    // The user said "simple mock".

    // We only update a subset to prevent seizure-inducing flickering
    for (mut spin, mut sprite) in &mut query {
        if rand::random::<f32>() < 0.05 {
            spin.0 *= -1;
            sprite.color = if spin.0 > 0 {
                Color::WHITE
            } else {
                Color::BLACK
            };
        }
    }
}
