use std::vec;

use crate::utils;

pub fn run_n_queens() {
    println!("Welcome to N Queens Backtracking");
    println!("Define the dimension of the gameboard");

    let n = utils::get_number();
    let mut board = vec![-1; n as usize];

    if backtracking(&mut board, 0, n) {
        print_board(&board, n);
    } else {
        println!("No solution found!");
    }
}

//changes made in board will persist after the funciton returns
fn backtracking(mut board: &mut Vec<i32>, row: usize, n: usize) -> bool {
    if row == n {
        return true;
    }

    for col in 0..n {
        if is_safe(&board, row, col) {
            board[row] = col as i32;

            if backtracking(&mut board, row + 1, n) {
                println!("Placed queen at row {}, col {}", row, col);
                return true;
            }

            board[row] = -1;
        }
    }

    return false;
}

fn is_safe(board: &[i32], row: usize, col: usize) -> bool {
    for i in 0..row {
        if board[i] == col as i32
            || board[i] - i as i32 == col as i32 - row as i32
            || board[i] + i as i32 == col as i32 + row as i32
        {
            return false;
        }
    }
    return true;
}

fn print_board(board: &Vec<i32>, n: usize) {
    println!("Solution found!");
    for row in 0..n {
        for col in 0..n {
            if board[row as usize] == col as i32 {
                print!("Q ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}
