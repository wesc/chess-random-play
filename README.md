# Chess Random Play

This is a library of random self play chess bots in various programming languages. In all the simulations we play to a maximum of 200 moves, at which point the game is declared a draw.

From the standard opening position, this results in approximately 11% of games ending as a win or loss, and 89% of games ending as draws. Without the 200 move cutoff, the win/loss rate is about 15% with an average number of moves, given a win/loss, of 160.


## Libraries

Each of the below libraries has its own folder in this repo.

- [Disservin's chess-library](https://github.com/Disservin/chess-library) (C++)
- [Jordan Bray's chess](https://github.com/jordanbray/chess) (Rust)
- [analog-hors's cozy_chess](https://github.com/analog-hors/cozy-chess) (Rust)
- [niklasf's python-chess](https://github.com/niklasf/python-chess) (Python)

Run `make clean run` to build and run 100,000 random play simulations (1,000 in the case of Python), and also output the stripped and compressed executable sizes if applicable.


## Benchmarks

Benchmarks are run w/ 1,000,000 simluations (except for Python) to get stable measurements.

C++ chess-library:

```
./simul 1000000
chess-library
-------------
Win/Loss: 108923
Draws: 891077
Average win/loss rate: 0.108923
Average draw rate: 0.891077
Average number of moves: 191.836
Average time per simulation: 5.26536e-05 seconds
Simulations per second: 18992
Total time elapsed: 52.6536 seconds
```

Rust chess crate:

```
./simul 1000000
jordanbray-chess
----------------
Win/Loss: 109622
Draws: 890378
Average win/loss rate: 10.96%
Average draw rate: 89.04%
Average number of moves: 191.87
Average time per simulation: 0.000060 seconds
Simulations per second: 16584.06
Total time elapsed: 60.30 seconds
```

Rust cozy_chess crate:

```
./simul 1000000
analog-hors-cozy-chess
----------------------
Win/Loss: 109356
Draws: 890644
Average win/loss rate: 10.94%
Average draw rate: 89.06%
Average number of moves: 191.91
Average time per simulation: 0.000051 seconds
Simulations per second: 19451.68
Total time elapsed: 51.41 seconds
```

Python chess:

```
.venv/bin/python simul.py 1000
niklasf-python-chess
--------------------
Win/Loss: 109
Draws: 891
Win/Loss Rate: 0.109
Draw Rate: 0.891
Average number of moves: 191.208
Average time per simulation: 0.010 seconds
Simulations per second: 97.243
Total time elapsed: 10.284 seconds
```

## License

Code in this repo in MIT licensed, unless otherwise specified. Chess libraries are authored and licensed on their own terms.
