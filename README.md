# Advent of Code in Rust

![Rust](https://github.com/aod/thrusted/workflows/Rust/badge.svg)

This project contains solutions for Advent of Code 2020 written in Rust.

# Quick start

1. Install the Rust programming language [here](https://www.rust-lang.org/tools/install).
2. Clone the repository: `git clone https://github.com/aod/thrusted.git`
2. Run a solution: `cargo run --release --bin <solver>`

## Example

```
$ cargo run --release --bin 20_23
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/20_23`
Part1(5.701µs):
24798635
Part2(1.505404239s):
12757828710
```

# Tests

This project contains at least tests on the example input(s) and real input for
every solution. To execute all tests run `cargo test --release`. You can also
run `cargo test --release example` to only run the example tests, replace
`example` with `answer` to test on real inputs. One can also specify the solver
by adding the `--bin <solver>` flag.
