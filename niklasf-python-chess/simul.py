import sys
import random
import time

import chess


WIN_LOSS = 0
DRAW = 1


def simul():
    move_count = 0
    board = chess.Board()
    while not board.is_game_over() and move_count < 200:
        move = random.choice(list(board.legal_moves))

        board.push(move)
        move_count += 1

    if board.is_game_over():
        if (
            board.is_checkmate()
            or board.is_stalemate()
            or board.is_insufficient_material()
        ):
            return (WIN_LOSS, move_count)

    return (DRAW, move_count)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python simul.py <num_simulations>")
        sys.exit(1)

    num_simulations = int(sys.argv[1])
    total_moves = 0
    win_loss = 0
    start_time = time.time()
    for _ in range(num_simulations):
        result, move_count = simul()
        total_moves += move_count
        if result == WIN_LOSS:
            win_loss += 1

    end_time = time.time()
    elapsed_time = end_time - start_time

    win_loss_rate = win_loss / num_simulations
    draw_rate = 1 - win_loss_rate
    average_moves = total_moves / num_simulations
    average_time_per_simulation = elapsed_time / num_simulations
    simulations_per_second = num_simulations / elapsed_time

    print(f"niklasf-python-chess")
    print(f"--------------------")
    print(f"Win/Loss: {win_loss}")
    print(f"Draws: {num_simulations - win_loss}")
    print(f"Win/Loss Rate: {win_loss_rate:.3f}")
    print(f"Draw Rate: {draw_rate:.3f}")
    print(f"Average number of moves: {average_moves:.3f}")
    print(f"Average time per simulation: {average_time_per_simulation:.3f} seconds")
    print(f"Simulations per second: {simulations_per_second:.3f}")
    print(f"Total time elapsed: {elapsed_time:.3f} seconds")
