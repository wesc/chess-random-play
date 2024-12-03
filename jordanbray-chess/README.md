# Random Play with Rust chess crate

This random play bot uses the Rust [chess crate](https://github.com/jordanbray/chess) from Jordan Bray.

## Build

This build attempts to minimize the binary size, and depends on Rust nightly. See [here](https://github.com/johnthagen/min-sized-rust) for background information.

```
# install Rust nightly toolchain
$ rustup toolchain install nightly
$ rustup component add rust-src --toolchain nightly
```

And then:

```
$ make
```

## Run

```
$ ./simul 1000
```

or

```
$ make run
```
