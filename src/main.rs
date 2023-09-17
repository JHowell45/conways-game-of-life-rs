pub mod board;
use bevy::prelude::*;
use board::{cell::CellState, Board};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, create_board)
        .add_systems(Update, update_board)
        .run();
}

fn create_board(mut commands: Commands) {
    let s: usize = 20;
    commands.spawn(Board::randomise(s, s));
}

fn update_board(query: Query<&mut Board>) {
    for board in &query {
        // board.step();
        println!("{}", board);
    }
}