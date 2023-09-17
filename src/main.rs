pub mod board;

use std::process::Command;
use board::Board;

fn main() {
    // let mut board = Board::randomise(4, 3);
    let s: usize = 20;
    let mut board = Board::randomise(s, s);
    println!("{}", board);
    loop {
        if let Err(_) = Command::new("clear").status() {
            let _ = Command::new("cls").status();
        }
        
        board.step();
        println!("\n\nConway's Game of Life\n\n");
        println!("X Size: {}", board.x_size);
        println!("Y Size: {}", board.y_size);
        println!();
        println!("{}", board);
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}
