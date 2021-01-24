# [Advent of Code][aoc] in [Rust][rust]

![Rust](https://github.com/aod/thrusted/workflows/Rust/badge.svg)

This project contains solutions for [Advent of Code 2020][aoc_2020] written in [Rust][rust].

# Quick start

1. Install the Rust programming language [here][rust_install].
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

To run all the tests execute the following command:

```
$ cargo test --release
```

The first non-flag argument in `test` is used to filter on the function names.
Use `example` to run example tests and `answer` for testing on real inputs.

One can use the `-p` and `--bin` flags as is used in the `cargo run` example
above to filter down to the year and or day.

# User benchmarks

Ran on Intel CPU i5-8250U (8) @ 3.400GHz.

## [Advent of Code 2020][aoc_2020]

| Day                | Part 1    | Part 2    |
|--------------------|----------:|----------:|
| [25][aoc_2020_25]  |  38.407ms |      N.A. |
| [24][aoc_2020_24]  | 596.084µs | 474.467ms |
| [23][aoc_2020_23]  |   5.885µs |   1.584s  |
| [22][aoc_2020_22]  |  15.182µs | 848.102ms |
| [21][aoc_2020_21]  |   1.302ms | 947.059µs |
| [20][aoc_2020_20]  |  16.124ms |   5.726ms |

[rust]: https://www.rust-lang.org
[rust_install]: https://www.rust-lang.org/tools/install
[aoc]: https://adventofcode.com
[aoc_2020]: https://adventofcode.com/2020
[aoc_2020_25]: https://adventofcode.com/2020/day/25
[aoc_2020_24]: https://adventofcode.com/2020/day/24
[aoc_2020_23]: https://adventofcode.com/2020/day/23
[aoc_2020_22]: https://adventofcode.com/2020/day/22
[aoc_2020_21]: https://adventofcode.com/2020/day/21
[aoc_2020_20]: https://adventofcode.com/2020/day/20
