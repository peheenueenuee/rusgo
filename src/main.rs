use std::fmt;
use std::io;

const BOARD_SIZE: usize = 3; // ボードの大きさを3x3に変更してマルバツゲーム向け

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
    Black, // X
    White, // O
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

    fn set_cell_state(&mut self, x: usize, y: usize, state: CellState) {
        if x < BOARD_SIZE && y < BOARD_SIZE {
            self.grid[y][x] = state;
        }
    }

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

    fn check_winner(&self) -> Option<Player> {
        let lines = [
            // Rows
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            // Columns
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            // Diagonals
            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)],
        ];
        
        for &line in &lines {
            let [a, b, c] = line;
            if self.grid[a.1][a.0] != CellState::Empty
                && self.grid[a.1][a.0] == self.grid[b.1][b.0]
                && self.grid[a.1][a.0] == self.grid[c.1][c.0]
            {
                return Some(match self.grid[a.1][a.0] {
                    CellState::Black => Player::Black,
                    CellState::White => Player::White,
                    _ => unreachable!(),
                });
            }
        }
        None
    }
}

fn get_user_input() -> Option<(usize, usize)> {
    let mut input = String::new();
    println!("Enter your move as 'x y': ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let coords: Vec<usize> = input
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    
    if coords.len() == 2 {
        Some((coords[0], coords[1]))
    } else {
        None
    }
}

fn main() {
    let mut board = Board::new();

    loop {
        board.display();
        if let Some((x, y)) = get_user_input() {
            if board.take_turn(x, y) {
                if let Some(winner) = board.check_winner() {
                    board.display();
                    println!("{} wins!", match winner {
                        Player::Black => "Black (X)",
                        Player::White => "White (O)",
                    });
                    break;
                }
            } else {
                println!("Invalid move, try again!");
            }
        } else {
            println!("Invalid input, please enter two numbers separated by space!");
        }
    }
}
