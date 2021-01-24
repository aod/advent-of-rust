# Advent of Code in Rust

![Rust](https://github.com/aod/thrusted/workflows/Rust/badge.svg)

This project contains solutions for Advent of Code 2020 written in Rust.

# Quick start

1. Install the Rust programming language [here](https://www.rust-lang.org/tools/install).
2. Clone the repository: `git clone https://github.com/aod/thrusted.git`
2. Run a solution: `cargo run --release -p aoc-2020 --bin <solver>`

## Example

```
$ cargo run -p aoc-2020 --release --bin 20
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/20`
Part1(17.098503ms):
19955159604613
Part2(9.77521ms):
1639
```

# Tests

This project contains at least tests on the example input(s) and real input for
every solution. To execute all tests run `cargo test --release`. You can also
run `cargo test --release example` to only run the example tests, replace
`example` with `answer` to test on real inputs. One can also specify the solver
by adding the `--bin <solver>` flag.

# Benchmarks

Host info:

```
*───────────────────────────────────────────────*
│ OS         =   Ubuntu 20.04.1 LTS             │
│ KERNEL     =   5.8.0-40-generic               │
│ CPU        =   Intel i5-8250U (8) @ 3.400GHz  │
│ PACKAGES   =   2423 (apt)                     │
│ MEMORY     =   4.07GiB / 7.66GiB              │
*───────────────────────────────────────────────*
```

## Advent of Code 2020

| Day | Part 1    | Part 2    |
|-----|----------:|----------:|
| 25  |  38.407ms |      N.A. |
| 24  | 596.084µs | 474.467ms |
| 23  |   5.885µs |   1.584s  |
| 22  |  15.182µs | 848.102ms |
| 21  |   1.302ms | 947.059µs |
| 20  |  16.124ms |   5.726ms |
