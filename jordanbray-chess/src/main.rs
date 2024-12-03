use std::env;
use std::time::Instant;

use chess::{Board, BoardStatus, MoveGen};
use rand::seq::IteratorRandom;

#[derive(PartialEq)]
enum GameResult {
    WinLoss,
    Draw,
}

fn simul() -> (GameResult, i32) {
    let mut board = Board::default();

    let mut move_count = 0;
    while board.status() == BoardStatus::Ongoing {
        let legal_moves = MoveGen::new_legal(&board);
        let mut rng = rand::thread_rng();
        let m = legal_moves.choose(&mut rng).unwrap();
        board = board.make_move_new(m);

        move_count += 1;
        if move_count > 200 {
            break;
        }
    }

    (
        match board.status() {
            BoardStatus::Checkmate => GameResult::WinLoss,
            BoardStatus::Stalemate => GameResult::Draw,
            _ => GameResult::Draw,
        },
        move_count,
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <num_simulations>", args[0]);
        std::process::exit(1);
    }
    let num_simulations: usize = args[1].parse().expect("Invalid number of simulations");

    let start = Instant::now();
    let mut total_moves = 0;
    let mut wins = 0;

    for _ in 0..num_simulations {
        let (result, moves) = simul();
        if result == GameResult::WinLoss {
            wins += 1;
        }
        total_moves += moves;
    }
    let duration = start.elapsed();
    let total_time_elapsed = duration.as_secs_f64();

    let draws = num_simulations - wins;
    let average_win_loss_rate = (wins as f64 / num_simulations as f64) * 100.0;
    let average_draw_rate = (draws as f64 / num_simulations as f64) * 100.0;
    let average_moves = total_moves as f64 / num_simulations as f64;
    let average_time_per_simulation = total_time_elapsed / num_simulations as f64;
    let simulations_per_second = num_simulations as f64 / total_time_elapsed;

    println!("jordanbray-chess");
    println!("----------------");
    println!("Win/Loss: {}", wins);
    println!("Draws: {}", draws);
    println!("Average win/loss rate: {:.2}%", average_win_loss_rate);
    println!("Average draw rate: {:.2}%", average_draw_rate);
    println!("Average number of moves: {:.2}", average_moves);
    println!(
        "Average time per simulation: {:.6} seconds",
        average_time_per_simulation
    );
    println!("Simulations per second: {:.2}", simulations_per_second);
    println!("Total time elapsed: {:.2} seconds", total_time_elapsed);
}
