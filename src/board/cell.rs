use std::convert::From;
use std::fmt;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Dead,
    Alive,
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Alive => write!(f, "1"),
            Self::Dead => write!(f, "0"),
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

impl Distribution<CellState> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CellState {
        match rng.gen_range(0..=1) {
            0 => CellState::Dead,
            _ => CellState::Alive,
        }
    }
}

impl From<bool> for CellState {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Alive,
            false => Self::Dead,
        }
    }
}

impl From<u8> for CellState {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Dead,
            1 => Self::Alive,
            _ => panic!("Invalid value for CellState! Must be either 0 or 1"),
        }
    }
}

impl From<u32> for CellState {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Dead,
            1 => Self::Alive,
            _ => panic!("Invalid value for CellState! Must be either 0 or 1"),
        }
    }
}

impl From<u64> for CellState {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Dead,
            1 => Self::Alive,
            _ => panic!("Invalid value for CellState! Must be either 0 or 1"),
        }
    }
}
