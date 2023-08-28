pub mod board;

use board::Board;

fn main() {
    let mut board = Board::randomise(4, 3);
    println!("{}", board);
    // board.step();
    for v in [0,1,3,5,6,10] {
        println!("{}", board.get_neighbours(v));
    }
}
