use crate::Board;
use crate::solver::bfs::{bfs};
use crate::solver::dfs::dfs;
use crate::solver::greedy::greedy;

mod dfs;
mod bfs;
mod greedy;
mod a_star;

pub fn is_valid(board: &Board, row: usize, col: usize, num: usize) -> bool {
    if board[row][col] != 0 {
        return false;
    }
    for i in 0..9 {
        if board[row][i] == num {
            return false;
        }
    }
    for i in 0..9 {
        if board[i][col] == num {
            return false;
        }
    }
    let sqrt_n = (9f64).sqrt() as usize;
    let start_row = (row / sqrt_n) * sqrt_n;
    let start_col = (col / sqrt_n) * sqrt_n;
    for i in 0..sqrt_n {
        for j in 0..sqrt_n {
            if board[start_row + i][start_col + j] == num {
                return false;
            }
        }
    }
    true
}

pub fn dfs_solver(board: &mut Board) -> bool {
    dfs(board)
}

pub fn bfs_solver(board: &mut Board) -> bool {
    bfs(board)
}

pub fn greedy_solver(board: &mut Board) -> bool {
    greedy(board)
}


