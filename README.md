# Chess Random Play

This is a library of random self play chess bots in various programming languages. In all the simulations we play to a maximum of 200 moves, at which point the game is declared a draw.

From the standard opening position, this results in approximately 11% of games ending as a win or loss, and 89% of games ending as draws. Without the 200 move cutoff, the win/loss rate is about 15% with an average number of moves, given a win/loss, of 160.


## Libraries

Each of the below libraries has its own folder in this repo.

- [Disservin's chess-library](https://github.com/Disservin/chess-library) (C++)
- [Jordan Bray's chess](https://github.com/jordanbray/chess) (Rust)

Run `make clean run` to build and run 10,000 random play simulations, and also output the stripped and compressed executable sizes if applicable.


## Benchmarks

chess-library:

```
-rwxrwxr-x 1 wesc wesc 72056 Dec  3 15:03 simul
-rw-rw-r-- 1 wesc wesc 33706 Dec  3 15:03 simul.gz
./simul 10000
chess-library
-------------
Win/Loss: 1065
Draws: 8935
Average win/loss rate: 0.1065
Average draw rate: 0.8935
Average number of moves: 191.787
Average time per simulation: 0.000718485 seconds
Simulations per second: 1391.82
Total time elapsed: 7.18485 seconds
```

Chess crate:

```
-rwxrwxr-x 1 wesc wesc 1003312 Dec  3 15:05 simul
-rw-rw-r-- 1 wesc wesc  113032 Dec  3 15:05 simul.gz
./simul 10000
jordanbray-chess
----------------
Win/Loss: 1113
Draws: 8887
Average win/loss rate: 11.13%
Average draw rate: 88.87%
Average number of moves: 191.63
Average time per simulation: 0.000057 seconds
Simulations per second: 17593.88
Total time elapsed: 0.57 seconds
```


## License

Code in this repo in MIT licensed, unless otherwise specified. Chess libraries are authored and licensed on their own terms.
