mod board;
mod solver;

use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc;
use crate::board::generate_sudoku;
use crate::solver::{dfs_solver, bfs_solver, greedy_solver};
pub type Board = Vec<Vec<usize>>;

fn main() {
    let k = 55; // Dificildade do jogo
    let num_tests = 10;

    // Canais para comunicação entre threads
    let (dfs_tx, dfs_rx) = mpsc::channel();
    let (bfs_tx, bfs_rx) = mpsc::channel();
    let (greedy_tx, greedy_rx) = mpsc::channel();

    // Marcadores de tempo total para cada algoritmo
    let mut total_time_dfs = Duration::new(0, 0);
    let mut total_time_bfs = Duration::new(0, 0);
    let mut total_time_greedy = Duration::new(0, 0);

    println!();

    // Thread para DFS
    println!("Realizando {} testes para DFS", num_tests);
    let start_total_dfs = Instant::now();
    let dfs_thread = thread::spawn(move || {
        let mut dfs_times = Vec::with_capacity(num_tests);
        for _ in 0..num_tests {
            let sudoku_puzzle = generate_sudoku(k);
            let mut sudoku_for_dfs = sudoku_puzzle.clone();
            let start_dfs = Instant::now();
            dfs_solver(&mut sudoku_for_dfs);
            let duration_dfs = start_dfs.elapsed();
            dfs_times.push(duration_dfs.as_secs_f64());
        }
        dfs_tx.send(dfs_times).expect("Falha ao enviar os tempos do DFS");
    });

    // Thread para BFS
    println!("Realizando {} testes para BFS", num_tests);
    let start_total_bfs = Instant::now();
    let bfs_thread = thread::spawn(move || {
        let mut bfs_times = Vec::with_capacity(num_tests);
        for _ in 0..num_tests {
            let sudoku_puzzle = generate_sudoku(k);
            let mut sudoku_for_bfs = sudoku_puzzle.clone();
            let start_bfs = Instant::now();
            bfs_solver(&mut sudoku_for_bfs);
            let duration_bfs = start_bfs.elapsed();
            bfs_times.push(duration_bfs.as_secs_f64());
        }
        bfs_tx.send(bfs_times).expect("Falha ao enviar os tempos do BFS");
    });

    // Thread para Greedy
    println!("Realizando {} testes para Greedy", num_tests);
    let start_total_greedy = Instant::now();
    let greedy_thread = thread::spawn(move || {
        let mut greedy_times = Vec::with_capacity(num_tests);
        for _ in 0..num_tests {
            let sudoku_puzzle = generate_sudoku(k);
            let mut sudoku_for_greedy = sudoku_puzzle.clone();
            let start_greedy = Instant::now();
            greedy_solver(&mut sudoku_for_greedy);
            let duration_greedy = start_greedy.elapsed();
            greedy_times.push(duration_greedy.as_secs_f64());
        }
        greedy_tx.send(greedy_times).expect("Falha ao enviar os tempos di Greedy");
    });

    println!("\n---------------------- Resultados ----------------------\n");
    // Aguardar threads terminarem e receber os tempos
    let dfs_times = dfs_rx.recv().expect("Falha ao receber os tempos do DFS");
    total_time_dfs = start_total_dfs.elapsed();
    let bfs_times = bfs_rx.recv().expect("Falha ao receber os tempos do BFS");
    total_time_bfs = start_total_bfs.elapsed();
    let greedy_times = greedy_rx.recv().expect("Falha ao receber os tempos do Greedy");
    total_time_greedy = start_total_greedy.elapsed();


    dfs_thread.join().expect("A thread DFS falhou");
    bfs_thread.join().expect("A thread BFS falhou");
    greedy_thread.join().expect("A thread do Greedy falhou");

    // Calcular média e desvio padrão para DFS
    let dfs_mean = mean(&dfs_times);
    let dfs_stddev = stddev(&dfs_times, dfs_mean);

    // Calcular média e desvio padrão para BFS
    let bfs_mean = mean(&bfs_times);
    let bfs_stddev = stddev(&bfs_times, bfs_mean);

    // Calcular média e desvio padrão para Greedy
    let greedy_mean = mean(&greedy_times);
    let greedy_stddev = stddev(&greedy_times, greedy_mean);


    println!("\nTempo médio para cada algoritimo: ");
    println!("DFS =    {:.4}s, Desvio padrão: {:.4}s", dfs_mean, dfs_stddev);
    println!("BFS =    {:.4}s, Desvio padrão: {:.4}s", bfs_mean, bfs_stddev);
    println!("GREEDY = {:.4}s, Desvio padrão: {:.4}s", greedy_mean, greedy_stddev);
    println!("\nTempo total para cada algoritimo:");
    println!("DFS:    {:.4}s", total_time_dfs.as_secs_f64());
    println!("BFS:    {:.4}s", total_time_bfs.as_secs_f64());
    println!("GREEDY: {:.4}s", total_time_greedy.as_secs_f64());

}

fn mean(times: &[f64]) -> f64 {
    let sum: f64 = times.iter().sum();
    sum / times.len() as f64
}
fn stddev(times: &[f64], mean: f64) -> f64 {
    let variance = times.iter().map(|time| {
        let diff = mean - time;
        diff * diff
    }).sum::<f64>() / times.len() as f64;
    variance.sqrt()
}