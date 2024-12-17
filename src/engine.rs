use crate::board::{Board, CellState, Player, BOARD_SIZE};

const WINNING_LENGTH: usize = 3; // 勝利条件の連続する石の数

fn find_line(grid: &[[CellState; BOARD_SIZE]; BOARD_SIZE], x: usize, y: usize, length: usize) -> bool {
    let directions = [(1, 0), (0, 1), (1, 1), (-1, 1)]; // 右、下、右下、左下

    for &(dx, dy) in &directions {
        let mut count = 1;
        let mut current_x = x as isize;
        let mut current_y = y as isize;

        while count < length {
            current_x += dx;
            current_y += dy;

            if current_x < 0
                || current_y < 0
                || current_x >= BOARD_SIZE as isize
                || current_y >= BOARD_SIZE as isize
            {
                break;
            }

            if grid[current_y as usize][current_x as usize] != grid[y][x] {
                break;
            }

            count += 1;
        }

        if count >= length {
            return true;
        }
    }
    false
}

pub fn check_winner(board: &Board) -> Option<&'static str> {
    let grid = board.get_grid();

    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if grid[y][x] == CellState::Empty {
                continue;
            }

            if find_line(grid, x, y, WINNING_LENGTH) {
                return match grid[y][x] {
                    CellState::Black => Some("Black (X)"),
                    CellState::White => Some("White (O)"),
                    _ => None,
                };
            }
        }
    }

    None
}
