use core::slice::Iter;

#[derive(Clone, Debug, PartialEq)]
pub struct Board(Vec<Vec<bool>>);

impl Board {
    pub fn new(size: usize) -> Self {
        Self(vec![vec![false; size]; size])
    }

    pub fn rows(&self) -> Iter<Vec<bool>> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn flip(&mut self, x: usize, y: usize) {
        let y_size: usize = self.len() -1;
        self.0[y_size - y][x] = !self.0[y_size - y][x]
    }

    pub fn display(&self) {
        for row in self.rows() {
            for cell in row.iter() {
                match cell {
                    false => print!(" {} ", 0),
                    true => print!(" {} ", 1),
                }
            }
            println!();
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_board() {
        let board = Board::new(3);
        assert_eq!(3, board.len());
        for row in board.rows() {
            assert_eq!(3, row.len());
        }
    }
}