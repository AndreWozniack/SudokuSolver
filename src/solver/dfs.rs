use crate::Board;
use crate::solver::is_valid;

pub fn dfs(board: &mut Board) -> bool {
    let n = board.len();
    for row in 0..n {
        for col in 0..n {
            if board[row][col] == 0 {
                for num in 1..=n {
                    if is_valid(board, row, col, num) {
                        board[row][col] = num;
                        if dfs(board) {
                            return true;
                        }
                        board[row][col] = 0;
                    }
                }
                return false;
            }
        }
    }
    true
}
