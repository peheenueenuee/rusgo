use std::fmt;

pub const BOARD_SIZE: usize = 7;

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Empty,
    Black,
    White,
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CellState::Empty => write!(f, "."),
            CellState::Black => write!(f, "X"),
            CellState::White => write!(f, "O"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn switch(self) -> Self {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }

    pub fn marker(self) -> CellState {
        match self {
            Player::Black => CellState::Black,
            Player::White => CellState::White,
        }
    }
}

pub struct Board {
    grid: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    pub current_player: Player,
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            current_player: Player::Black,
        }
    }

    pub fn set_cell_state(&mut self, x: usize, y: usize, state: CellState) {
        if x < BOARD_SIZE && y < BOARD_SIZE {
            self.grid[y][x] = state;
        }
    }

    pub fn take_turn(&mut self, x: usize, y: usize) -> bool {
        if x < BOARD_SIZE && y < BOARD_SIZE && self.grid[y][x] == CellState::Empty {
            self.set_cell_state(x, y, self.current_player.marker());
            self.current_player = self.current_player.switch();
            true
        } else {
            false
        }
    }

    pub fn display(&self) {
        for row in &self.grid {
            for &cell in row {
                print!("{} ", cell);
            }
            println!();
        }
    }

    pub fn get_grid(&self) -> &[[CellState; BOARD_SIZE]; BOARD_SIZE] {
        &self.grid
    }
}
