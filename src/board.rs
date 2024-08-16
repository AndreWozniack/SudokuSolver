use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use crate::Board;
use crate::solver::is_valid;


fn generate_complete_sudoku() -> Board {
    let mut board = vec![vec![0; 9]; 9];
    fill_sudoku(&mut board);
    board
}

fn fill_sudoku(board: &mut Board) -> bool {
    let mut rng = thread_rng();
    let numbers: Vec<u32> = (1..=9 as u32).collect();

    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] == 0 {
                let mut shuffled_numbers = numbers.clone();
                shuffled_numbers.shuffle(&mut rng);

                for &num in shuffled_numbers.iter() {
                    if is_valid(board, row, col, num as usize) {
                        board[row][col] = num as usize;
                        if fill_sudoku(board) {
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

fn remove_numbers(board: &mut Board, k: usize) {
    let mut rng = thread_rng();
    let mut removed = 0;

    while removed < k {
        let row = rng.gen_range(0..9);
        let col = rng.gen_range(0..9);

        if board[row][col] != 0 {
            board[row][col] = 0;
            removed += 1;
        }
    }
}

pub fn generate_sudoku(k: usize) -> Board {
    // Gera um tabuleiro completo de Sudoku
    let mut board = generate_complete_sudoku();

    // Remove k números aleatórios para criar o quebra-cabeça
    remove_numbers(&mut board, k);

    board
}

