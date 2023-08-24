pub mod board;

use board::Board;

fn main() {
    let mut board = Board::randomise(10);
    board.display();

    board.flip(0, 0);
    board.display();

    board.flip(0, 0);
    board.display();

    board.flip(1, 1);
    board.display();
}
