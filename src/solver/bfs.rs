use std::collections::{VecDeque, HashSet};
use crate::Board;
use crate::solver::is_valid;


pub fn bfs(board: &mut Board) -> bool {
    let n = board.len();
    let mut queue = VecDeque::new();
    queue.push_back(board.clone());

    while let Some(current_board) = queue.pop_front() {
        if is_solved(&current_board) {
            *board = current_board;
            return true;
        }

        'outer: for row in 0..n {
            for col in 0..n {
                if current_board[row][col] == 0 {
                    for num in 1..=n {
                        if is_valid(&current_board, row, col, num) {
                            let mut new_board = current_board.clone();
                            new_board[row][col] = num;
                            queue.push_back(new_board);
                        }
                    }
                    break 'outer;
                }
            }
        }
    }
    false
}

fn is_solved(board: &Board) -> bool {
    for row in board {
        for &val in row {
            if val == 0 {
                return false;
            }
        }
    }
    true
}