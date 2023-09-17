pub mod board;

use board::Board;

fn main() {
    // let mut board = Board::randomise(4, 3);
    let mut board = Board::from_u8_vec(vec![0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0], Some(3));
    println!("{}", board);
    println!("x_size: {}", board.x_size);
    println!("y_size: {}", board.y_size);
    for index in 0..board.size() {
        board.get_neighbours(index);
    }
    board.step();
    println!("{}", board);
}
