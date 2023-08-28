use std::fmt;
use rand::{
    distributions::{Distribution, Standard},
    Rng
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Alive,
    Dead
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Alive => write!(f, "1"),
            Self::Dead => write!(f, "0"),
        }
    }
}

impl Distribution<CellState> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CellState {
        match rng.gen_range(0..=1) {
            0 => CellState::Dead,
            _ => CellState::Alive
        }
    }
}

impl fmt::Debug for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Alive => write!(f, "1"),
            Self::Dead => write!(f, "0"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    data: Vec<CellState>,
    pub x_size: usize,
    pub y_size: usize
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
            y_size: y
        }
    }

    pub fn square(x: usize) -> Self {
        Self::new(x, x)
    }

    pub fn randomise(x: usize, y: usize) -> Self {
        Self {
            data: (0..(x * y)).map(|_| rand::random::<CellState>()).collect(),
            x_size: x,
            y_size: y
        }
    }

    pub fn randomise_square(x: usize) -> Self {
        Self::randomise(x, x)
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

        println!("Index: {}", index);
        println!("Row: {}", row);

        if index == 0 {
            if self.data[index + 1] == CellState::Alive {
                neighbours += 1;
            }
            if self.data[index + self.x_size] == CellState::Alive {
                neighbours += 1;
            }
            if self.data[index + self.x_size + 1] == CellState::Alive {
                neighbours += 1;
            }
        } else if index == self.size() - 1 {
            if self.data[index - 1] == CellState::Alive {
                neighbours += 1;
            }
            if self.data[index - self.x_size] == CellState::Alive {
                neighbours += 1;
            }
            if self.data[index - self.x_size - 1] == CellState::Alive {
                neighbours += 1;
            }
        } else if index + 1 % self.x_size == 0 {
            if self.data[index - 1] == CellState::Alive {
                neighbours += 1;
            }
            if row >= 1 {
                if self.data[index - self.x_size] == CellState::Alive {
                    neighbours += 1;
                }
                if self.data[index - self.x_size-1] == CellState::Alive {
                    neighbours += 1;
                }
            }
            if row < self.y_size {
                if self.data[index + self.x_size] == CellState::Alive {
                    neighbours += 1;
                }
                if self.data[index + self.x_size + 1] == CellState::Alive {
                    neighbours += 1;
                }
            }
        } else {

        }
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

    #[test]
    fn test_create_board() {
        let board = Board::new(3, 3);
        assert_eq!(9, board.size());
    }
}