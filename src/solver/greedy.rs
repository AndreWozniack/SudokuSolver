use crate::Board;
use crate::solver::is_valid;

/// Resolve o Sudoku utilizando um algoritmo combinado de guloso e backtracking.
/// Retorna `true` se o Sudoku for resolvido com sucesso, caso contrário, `false`.
pub fn greedy(board: &mut Board) -> bool {
    let n = board.len();

    // Procura pela primeira célula vazia no tabuleiro
    if let Some((row, col)) = find_empty_cell(board) {
        // Tenta preencher a célula vazia com números de 1 a 9
        for num in 1..=n {
            if is_valid(board, row, col, num) {
                board[row][col] = num; // Insere o número na célula

                // Continua tentando resolver o resto do tabuleiro
                if greedy(board) {
                    return true;
                }

                // Se não conseguir resolver com esse número, limpa a célula (backtrack)
                board[row][col] = 0;
            }
        }

        // Se nenhum número de 1 a 9 funcionar, retorna falso para indicar falha
        false
    } else {
        // Se não houver células vazias, então o Sudoku foi resolvido
        true
    }
}

/// Encontra a primeira célula vazia no tabuleiro e retorna sua posição.
fn find_empty_cell(board: &Board) -> Option<(usize, usize)> {
    let n = board.len();
    for row in 0..n {
        for col in 0..n {
            if board[row][col] == 0 {
                return Some((row, col));
            }
        }
    }
    None
}
