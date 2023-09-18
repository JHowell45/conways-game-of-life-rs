pub mod board;
use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Material2d}};
use board::{cell::CellState, Board};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GameStatePlugin))
        .run();
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameIntervalTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, create_board)
        .add_systems(Update, draw);
    }
}

fn create_board(mut commands: Commands) {
    let s: usize = 40;
    commands.spawn(Board::randomise(s, s));
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource)]
struct GameIntervalTimer(Timer);

fn draw(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut timer: ResMut<GameIntervalTimer>,
    mut board: Query<&mut Board>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut board = board.single_mut();
        board.step();
        let radius = 10.;
        let radius_buffer = (radius * 2.) + 2.;

        let x_size = radius_buffer * board.x_size as f32;
        let y_size = radius_buffer * board.y_size as f32;
        let x_start = 0. - (x_size / 2.);
        let y_start = 0. - (y_size / 2.);

        let mut x = x_start;
        let mut y = y_start;

        let x_step = radius_buffer;
        let y_step = radius_buffer;

        println!("x_size: {}", x_size);
        println!("y_size: {}", y_size);
        println!("x_start: {}", x_start);
        println!("y_start: {}", y_start);
        println!("x: {}", x);
        println!("y: {}", y);
        println!("radius: {}", radius);
        println!("x_step: {}", x_step);
        println!("y_step: {}", y_step);

        meshes.clear();
        materials.clear();

        for (index, value) in board.data.iter().enumerate() {
            if index % board.x_size == 0 {
                x = x_start;
                y += y_step;
            }
            let colour = match value {
                CellState::Alive => Color::RED,
                CellState::Dead => Color::GRAY,
            };
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(radius).into()).into(),
                material: materials.add(ColorMaterial::from(colour)),
                transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                ..default()
            });
            x += x_step;
        }
    }
}
