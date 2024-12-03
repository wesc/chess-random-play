#include <chrono>
#include <random>
#include <algorithm>
#include <memory>

#include "chess.hpp"

std::pair<chess::GameResult, int> random_simul()
{
    auto board = std::make_shared<chess::Board>();

    int move_count = 0;
    while (true)
    {
        auto result = board->isGameOver();
        if (result.second != chess::GameResult::NONE)
        {
            return std::make_pair(result.second, move_count);
        }

        chess::Movelist movelist;
        chess::movegen::legalmoves(movelist, *board);
        std::random_device rd;
        std::mt19937 gen(rd());
        std::uniform_int_distribution<> dis(0, movelist.size() - 1);
        auto move = movelist[dis(gen)];
        board->makeMove(move);

        move_count++;
        if (move_count > 200)
        {
            break;
        }
    }

    return std::make_pair(chess::GameResult::DRAW, move_count);
}

int main(int argc, char const *argv[])
{
    if (argc < 2)
    {
        std::cerr << "Usage: " << argv[0] << " <num_simulations>" << std::endl;
        return 1;
    }

    int num_simulations = std::stoi(argv[1]);
    int win_loss = 0, draws = 0, total_moves = 0;

    auto start_time = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < num_simulations; ++i)
    {
        auto result = random_simul();
        switch (result.first)
        {
        case chess::GameResult::WIN:
            win_loss++;
            break;
        case chess::GameResult::LOSE:
            win_loss++;
            break;
        case chess::GameResult::DRAW:
            draws++;
            break;
        default:
            break;
        }
        total_moves += result.second;
    }
    auto end_time = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end_time - start_time;

    double average_win_loss_rate = static_cast<double>(win_loss) / num_simulations;
    double average_draw_rate = static_cast<double>(draws) / num_simulations;
    double average_moves = static_cast<double>(total_moves) / num_simulations;
    double average_time_per_simulation = elapsed.count() / num_simulations;
    double simulations_per_second = num_simulations / elapsed.count();

    std::cout << "chess-library" << std::endl;
    std::cout << "-------------" << std::endl;
    std::cout << "Win/Loss: " << win_loss << std::endl;
    std::cout << "Draws: " << draws << std::endl;
    std::cout << "Average win/loss rate: " << average_win_loss_rate << std::endl;
    std::cout << "Average draw rate: " << average_draw_rate << std::endl;
    std::cout << "Average number of moves: " << average_moves << std::endl;
    std::cout << "Average time per simulation: " << average_time_per_simulation << " seconds" << std::endl;
    std::cout << "Simulations per second: " << simulations_per_second << std::endl;
    std::cout << "Total time elapsed: " << elapsed.count() << " seconds" << std::endl;

    return 0;
}
