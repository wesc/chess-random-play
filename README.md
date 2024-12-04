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

C++ chess-library:

```
-rwxrwxr-x 1 wesc wesc 72056 Dec  3 19:49 simul
-rw-rw-r-- 1 wesc wesc 33782 Dec  3 19:52 simul.gz
./simul 100000
chess-library
-------------
Win/Loss: 10959
Draws: 89041
Average win/loss rate: 0.10959
Average draw rate: 0.89041
Average number of moves: 191.763
Average time per simulation: 5.37462e-05 seconds
Simulations per second: 18606
Total time elapsed: 5.37462 seconds
```

Rust chess crate:

```
-rwxrwxr-x 1 wesc wesc 992248 Dec  3 20:17 simul
-rw-rw-r-- 1 wesc wesc 107288 Dec  3 20:17 simul.gz
./simul 100000
jordanbray-chess
----------------
Win/Loss: 11051
Draws: 88949
Average win/loss rate: 11.05%
Average draw rate: 88.95%
Average number of moves: 191.80
Average time per simulation: 0.000058 seconds
Simulations per second: 17314.62
Total time elapsed: 5.78 seconds
```

Rust cozy_chess crate:

```
analog-hors-cozy-chess
----------------------
Win/Loss: 10913
Draws: 89087
Average win/loss rate: 10.91%
Average draw rate: 89.09%
Average number of moves: 191.95
Average time per simulation: 0.000076 seconds
Simulations per second: 13083.95
Total time elapsed: 7.64 seconds
```

Python chess (note this run is for 1k simulations, not 100k like the C++ and Rust libraries):

```
-rw-rw-r-- 1 wesc wesc 1789 Dec  3 20:41 simul.py
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
