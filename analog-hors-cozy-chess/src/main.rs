use std::env;
use std::time::Instant;

use rand::rngs::SmallRng;
use rand::seq::IteratorRandom;
use rand::SeedableRng;

#[derive(PartialEq)]
enum GameResult {
    WinLoss,
    Draw,
}

fn simul(rng: &mut SmallRng) -> (GameResult, i32) {
    let mut board = cozy_chess::Board::default();
    let mut move_count = 0;
    let mut move_list = Vec::new();

    while board.status() == cozy_chess::GameStatus::Ongoing {
        move_list.clear();
        board.generate_moves(|moves| {
            move_list.extend(moves);
            false
        });

        if move_list.is_empty() {
            break;
        }

        let m = *move_list.iter().choose(rng).unwrap();
        board.play_unchecked(m);

        move_count += 1;
        if move_count > 200 {
            break;
        }
    }

    (
        match board.status() {
            cozy_chess::GameStatus::Won => GameResult::WinLoss,
            cozy_chess::GameStatus::Drawn => GameResult::Draw,
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

    let mut rng = rand::rngs::SmallRng::from_entropy();
    for _ in 0..num_simulations {
        let (result, moves) = simul(&mut rng);
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

    println!("analog-hors-cozy-chess");
    println!("----------------------");
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
