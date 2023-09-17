pub mod board;

use board::Board;

fn main() {
    // let mut board = Board::randomise(4, 3);
    let mut board = Board::from_vec(
        vec![
            board::CellState::Dead,
            board::CellState::Alive,
            board::CellState::Dead,
            board::CellState::Dead,
            board::CellState::Alive,
            board::CellState::Alive,
            board::CellState::Dead,
            board::CellState::Alive,
            board::CellState::Dead,
            board::CellState::Dead,
            board::CellState::Alive,
            board::CellState::Dead,
        ],
        Some(3),
    );
    println!("{}", board);
    println!("x_size: {}", board.x_size);
    println!("y_size: {}", board.y_size);
    for index in 0..board.size() {
        board.get_neighbours(index);
    }
}
