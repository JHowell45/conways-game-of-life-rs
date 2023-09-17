mod cell;
use std::fmt;

use cell::CellState;

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    data: Vec<CellState>,
    pub x_size: usize,
    pub y_size: usize,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for row in self.data.chunks(self.x_size) {
            for cell in row {
                output.push_str(&format!(" {} ", cell));
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}

impl Board {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            data: vec![CellState::Dead; x * y],
            x_size: x,
            y_size: y,
        }
    }

    pub fn square(x: usize) -> Self {
        Self::new(x, x)
    }

    pub fn randomise(x: usize, y: usize) -> Self {
        Self {
            data: (0..(x * y)).map(|_| rand::random::<CellState>()).collect(),
            x_size: x,
            y_size: y,
        }
    }

    pub fn randomise_square(x: usize) -> Self {
        Self::randomise(x, x)
    }

    pub fn from_vec(data: Vec<CellState>, rows: Option<usize>) -> Self {
        let rows = match rows {
            Some(rows) => rows,
            None => (data.len() as f32).sqrt() as usize,
        };
        let cols = data.len() / rows;
        if data.len() % rows != 0 {
            panic!(
                "Invalid rows for data length of {} for row size of {}",
                data.len(),
                rows
            );
        }
        Self {
            data,
            x_size: cols,
            y_size: rows,
        }
    }

    pub fn from_u8_vec(data: Vec<u8>, rows: Option<usize>) -> Self {
        let rows = match rows {
            Some(rows) => rows,
            None => (data.len() as f32).sqrt() as usize,
        };
        let cols = data.len() / rows;
        if data.len() % rows != 0 {
            panic!(
                "Invalid rows for data length of {} for row size of {}",
                data.len(),
                rows
            );
        }
        Self {
            data: data.into_iter().map(|x| CellState::from(x)).collect(),
            x_size: cols,
            y_size: rows,
        }
    }

    pub fn size(&self) -> usize {
        self.x_size * self.y_size
    }

    pub fn handle_flip(&mut self, index: usize, value: CellState, neighbours: usize) {
        match value {
            CellState::Alive => {
                if neighbours != 2 && neighbours != 3 {
                    self.data[index] = CellState::Dead
                }
            }
            CellState::Dead => {
                if neighbours == 3 {
                    self.data[index] = CellState::Alive
                }
            }
        }
    }

    pub fn get_neighbours(&self, index: usize) -> usize {
        let row = index / self.x_size;
        let mut neighbours: usize = 0;

        if index == 0 {
            println!("(1) TRUE START || Index: {}", index);
            neighbours += self.data[index + 1] as usize;
            neighbours += self.data[index + self.x_size] as usize;
            neighbours += self.data[index + self.x_size + 1] as usize;
        } else if index == self.x_size - 1 {
            println!("(2) END OF THE START ROW || Index: {}", index);
            neighbours += self.data[index - 1] as usize;
            neighbours += self.data[index + self.x_size] as usize;
            neighbours += self.data[index + self.x_size - 1] as usize;
        } else if index == self.size() - 1 {
            println!("(3) TRUE END || Index: {}", index);
            neighbours += self.data[index - 1] as usize;
            neighbours += self.data[index - self.x_size] as usize;
            neighbours += self.data[index - self.x_size - 1] as usize;
        } else if index == (self.size() - self.x_size) {
            println!("(5) LAST ROW START || Index: {}", index);
            neighbours += self.data[index + 1] as usize;
            neighbours += self.data[index - self.x_size] as usize;
            neighbours += self.data[index - self.x_size + 1] as usize;
        } else if (index != 0 || index != self.size()-1) && (index % self.x_size == 0) {
            println!("(5) START OF ALL OTHER ROWS || Index: {}", index);
            neighbours += self.data[index + 1] as usize;
            neighbours += self.data[index + self.x_size] as usize;
            neighbours += self.data[index + self.x_size + 1] as usize;
            neighbours += self.data[index - self.x_size] as usize;
            neighbours += self.data[index - self.x_size + 1] as usize;
        } else if index != 1 && ((index + 1) % self.x_size == 0) {
            println!("(6) END OF ALL OTHER ROWS || Index: {}", index);
            neighbours += self.data[index + 1] as usize;
            neighbours += self.data[index - self.x_size] as usize;
            neighbours += self.data[index - self.x_size - 1] as usize;
            neighbours += self.data[index + self.x_size] as usize;
            neighbours += self.data[index + self.x_size - 1] as usize;
        } else {
            println!("OTHER || Index: {}", index);
        }
        println!(
            "x: {}, y: {} || value: {} || index: {} || neighbours: {}",
            index % self.x_size,
            row,
            self.data[index],
            index,
            neighbours
        );
        neighbours
    }

    pub fn step(&mut self) {
        for (index, val) in self.data.iter_mut().enumerate() {
            println!("({}, {})", index, val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_create_board() {
        let board = Board::new(3, 3);
        assert_eq!(9, board.size());
    }

    #[test]
    fn test_from_vec_no_rows() {
        let board = Board::from_vec(
            vec![
                CellState::Dead,
                CellState::Dead,
                CellState::Alive,
                CellState::Dead,
            ],
            None,
        );
        assert_eq!(2, board.x_size);
        assert_eq!(2, board.y_size);
    }

    #[test]
    fn test_from_vec_with_rows() {
        let board = Board::from_vec(
            vec![
                CellState::Dead,
                CellState::Dead,
                CellState::Alive,
                CellState::Dead,
                CellState::Dead,
                CellState::Dead,
            ],
            Some(3),
        );
        assert_eq!(2, board.x_size);
        assert_eq!(3, board.y_size);
    }

    #[test]
    #[should_panic]
    fn test_from_vec_no_rows_invalid_data_size() {
        Board::from_vec(
            vec![
                CellState::Dead,
                CellState::Dead,
                CellState::Alive,
                CellState::Dead,
                CellState::Dead,
            ],
            None,
        );
    }

    #[test]
    #[should_panic]
    fn test_from_vec_with_rows_invalid_data_size() {
        Board::from_vec(
            vec![
                CellState::Dead,
                CellState::Dead,
                CellState::Alive,
                CellState::Dead,
                CellState::Dead,
            ],
            Some(2),
        );
    }

    #[test_case(vec![
        CellState::Dead,
        CellState::Alive,
        CellState::Dead,
        CellState::Alive,
        CellState::Alive,
        CellState::Dead,
        CellState::Dead,
        CellState::Dead,
        CellState::Dead,
    ], None, 3, 3, 0, 3 ; "3x3 grid with 3 neighbours for index 0")]
    #[test_case(vec![
        CellState::Dead,
        CellState::Alive,
        CellState::Dead,
        CellState::Alive,
        CellState::Dead,
        CellState::Dead,
        CellState::Dead,
        CellState::Dead,
        CellState::Dead,
    ], None, 3, 3, 0, 2 ; "3x3 grid with 2 neighbours for index 0")]
    #[test_case(vec![
        CellState::Alive,
        CellState::Alive,
        CellState::Alive,
        CellState::Alive,
        CellState::Dead,
        CellState::Dead,
        CellState::Dead,
        CellState::Dead,
        CellState::Dead,
    ], None, 3, 3, 1, 3 ; "3x3 grid with 2 neighbours for index 1")]
    #[test_case(vec![
        CellState::Alive,
        CellState::Alive,
        CellState::Alive,
        CellState::Alive,
        CellState::Alive,
        CellState::Alive,
        CellState::Dead,
        CellState::Alive,
        CellState::Dead,
    ], None, 3, 3, 8, 3 ; "3x3 grid with 2 neighbours for index 8")]
    #[test_case(vec![
        CellState::Alive,
        CellState::Alive,
        CellState::Alive,
        CellState::Alive,
        CellState::Alive,
        CellState::Alive,
        CellState::Dead,
        CellState::Alive,
        CellState::Dead,
    ], None, 3, 3, 4, 4 ; "3x3 grid with 2 neighbours for index 4")]
    fn test_get_neighbours(
        data: Vec<CellState>,
        rows: Option<usize>,
        x_size: usize,
        y_size: usize,
        index: usize,
        neighbours: usize,
    ) {
        let board = Board::from_vec(data, rows);
        println!("{}", board);
        assert_eq!(x_size, board.x_size);
        assert_eq!(y_size, board.y_size);
        assert_eq!(neighbours, board.get_neighbours(index));
    }
}
