use crate::board::{Board, CellState, Player, BOARD_SIZE};

pub fn check_winner(board: &Board) -> Option<&'static str> {
    let grid = board.get_grid();
    let lines = [
        [(0, 0), (0, 1), (0, 2)],
        [(1, 0), (1, 1), (1, 2)],
        [(2, 0), (2, 1), (2, 2)],
        [(0, 0), (1, 0), (2, 0)],
        [(0, 1), (1, 1), (2, 1)],
        [(0, 2), (1, 2), (2, 2)],
        [(0, 0), (1, 1), (2, 2)],
        [(0, 2), (1, 1), (2, 0)],
    ];

    for &line in &lines {
        let [a, b, c] = line;
        if grid[a.1][a.0] != CellState::Empty
            && grid[a.1][a.0] == grid[b.1][b.0]
            && grid[a.1][a.0] == grid[c.1][c.0]
        {
            return match grid[a.1][a.0] {
                CellState::Black => Some("Black (X)"),
                CellState::White => Some("White (O)"),
                _ => None,
            };
        }
    }
    None
}
