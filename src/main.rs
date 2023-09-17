pub mod board;

use board::Board;

fn main() {
    // let mut board = Board::randomise(4, 3);
    let s: usize = 20;
    let mut board = Board::randomise(s, s);
    println!("{}", board);
    loop {
        board.step();
        println!("{}", board);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
