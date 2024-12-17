mod board;
mod engine;

use board::Board;
use std::io;
use engine::check_winner;

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
                if let Some(winner) = check_winner(&board) {
                    board.display();
                    println!("{} wins!", winner);
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
