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

#[derive(Clone, Copy)]
enum Player {
    Black,
    White,
}

impl Player {
    fn switch(self) -> Self {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }

    fn marker(self) -> CellState {
        match self {
            Player::Black => CellState::Black,
            Player::White => CellState::White,
        }
    }
}

struct Board {
    grid: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    current_player: Player,
}

impl Board {
    fn new() -> Self {
        Self {
            grid: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            current_player: Player::Black,
        }
    }

    // セルの状態を設定するだけの関数
    fn set_cell_state(&mut self, x: usize, y: usize, state: CellState) {
        if x < BOARD_SIZE && y < BOARD_SIZE {
            self.grid[y][x] = state;
        }
    }

    // プレイヤーが手番を行うラッパー関数
    fn take_turn(&mut self, x: usize, y: usize) -> bool {
        if x < BOARD_SIZE && y < BOARD_SIZE && self.grid[y][x] == CellState::Empty {
            self.set_cell_state(x, y, self.current_player.marker());
            self.current_player = self.current_player.switch();
            true
        } else {
            false
        }
    }

    fn display(&self) {
        for row in &self.grid {
            for &cell in row {
                print!("{} ", cell);
            }
            println!();
        }
        println!("Current player: {}", match self.current_player {
            Player::Black => "Black (X)",
            Player::White => "White (O)",
        });
    }
}

fn main() {
    let mut board = Board::new();
    
    // デモ: プレイヤー交互に手を打つ
    board.take_turn(2, 2);
    board.take_turn(3, 3);
    board.take_turn(1, 1);
    board.take_turn(0, 0);

    // ボードを表示
    board.display();
}
