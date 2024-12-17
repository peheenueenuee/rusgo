use std::fmt;

const BOARD_SIZE: usize = 5;

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Empty,       // 空の状態
    Black,       // 黒の駒
    White,       // 白の駒
    Unavailable, // 使用できない状態
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CellState::Empty => write!(f, "."),
            CellState::Black => write!(f, "X"),
            CellState::White => write!(f, "O"),
            CellState::Unavailable => write!(f, "#"),
        }
    }
}

struct Board {
    grid: [[CellState; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn new() -> Self {
        Self {
            grid: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    fn set_cell(&mut self, x: usize, y: usize, state: CellState) {
        if x < BOARD_SIZE && y < BOARD_SIZE {
            self.grid[y][x] = state;
        }
    }

    fn display(&self) {
        for row in &self.grid {
            for &cell in row {
                print!("{} ", cell);
            }
            println!();
        }
    }
}

fn main() {
    let mut board = Board::new();
    
    // デモ
    board.set_cell(2, 2, CellState::Black);
    board.set_cell(3, 3, CellState::White);
    board.set_cell(4, 4, CellState::Unavailable);

    // ボードを表示
    board.display();
}
