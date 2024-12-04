# Chess Random Play

This is a library of random self play chess bots in various programming languages. In all the simulations we play to a maximum of 200 moves, at which point the game is declared a draw.

From the standard opening position, this results in approximately 11% of games ending as a win or loss, and 89% of games ending as draws. Without the 200 move cutoff, the win/loss rate is about 15% with an average number of moves, given a win/loss, of 160.


## Libraries

Each of the below libraries has its own folder in this repo.

- [Disservin's chess-library](https://github.com/Disservin/chess-library) (C++)
- [Jordan Bray's chess](https://github.com/jordanbray/chess) (Rust)

Run `make clean run` to build and run 100,000 random play simulations, and also output the stripped and compressed executable sizes if applicable.


## Benchmarks

chess-library:

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

Chess crate:

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


## License

Code in this repo in MIT licensed, unless otherwise specified. Chess libraries are authored and licensed on their own terms.
