# Advent of Code in Rust

This project contains solutions for Advent of Code 2020 written in Rust.

# Quick start

1. Install the Rust programming language [here](https://www.rust-lang.org/tools/install).
2. Clone the repository: `git clone https://github.com/aod/thrusted.git`
2. Run a solution: `cargo run <solver> <input_file_path>`

## Example

```
$ cargo run --release 22 aoc-2020/inputs/22.txt
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/aoc-2020 22 aoc-2020/inputs/22.txt`
Part1(17.113µs):
33010
Part2(656.091976ms):
32769
```

# Solvers

A solver may simply represent a solution for a day in an Advent of Code year.
Or it may represent a solver for the animated variant, or an optimized one. The
idea here is that there isn't necessarily a single solver for a day, there may
be multiple with variants.

Take for example the [2019 Day 13: Care Package](https://adventofcode.com/2019/day/13) puzzle.
In a nutshell it's about simulating an existing game called Breakout. The extra
challenge for this codebase is to be capable of providing the option of having
just the solver logging the answer and whatever extra variant. For example an
interactable version where the user can play the Breakout game instead.

# Testing

Every solved puzzle has at least tests for the example and real input. These
tests are ran using `cargo test` which are also automatically performed on the
CI when a commit or pull request has happened to the main branch.
