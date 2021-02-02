# [Advent of Code][aoc] in [Rust][rust]

![Rust](https://github.com/aod/thrusted/workflows/Rust/badge.svg)

This project (for the time being) only contains solutions for
[Advent of Code 2020][aoc_2020].

# Quick start

1. Install the Rust programming language [here][rust_install].
2. Clone the repository: `git clone https://github.com/aod/advent-of-unwrap.git`
2. Run a solution: `cargo run --release -p aoc-<year> --bin <day>`

## Example

```
$ cargo run --release -p aoc-2020 --bin 20
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/20`
Part1(17.098503ms):
19955159604613
Part2(9.77521ms):
1639
```

# Tests

Every example and user input is tested and automatically run in CI using GitHub
Actions. To run all tests locally execute the following command:

```
$ cargo test --release
```

The first non-flag argument in `cargo test` is used to filter on the function
name. Use `example` or `answer` to test solutions with the example or user input
respectively.

One can further filter down to the year and or day by adding the previously
mentioned `-p` and or `--bin` flags, as is used in the `cargo run` example above.

# User benchmarks

Ran on Intel CPU i5-8250U (8) @ 3.400GHz.

## [Advent of Code 2020][aoc_2020]

| Day                | Part 1     | Part 2     | Sum        |
|--------------------|-----------:|-----------:|-----------:|
| [25][aoc_2020_25]  |  38.407 ms |   *N.A.*   |  38.407 ms |
| [24][aoc_2020_24]  | 596.084 µs | 474.467 ms | 475.036 ms |
| [23][aoc_2020_23]  |   5.885 µs |   1.584  s |   1.583  s |
| [22][aoc_2020_22]  |  15.182 µs | 848.102 ms | 848.117 ms |
| [21][aoc_2020_21]  |   1.302 ms | 947.059 µs |   2.249 ms |
| [20][aoc_2020_20]  |  16.124 ms |   5.726 ms |  21.850 ms |
| [19][aoc_2020_19]  |  *N.A.*    |  *N.A.*    |  *N.A.*    |
| [18][aoc_2020_18]  | 877.271 µs |   1.752 ms |   2.629 ms |
| Total              |  57.327 ms |   2.915  s |   2.971  s |

---

<!-- find . -name "*.rs" -exec grep -Hn ".unwrap()" {} 2>/dev/null \; | wc -l -->
`.unwrap()` counter: **20**

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
[aoc_2020_19]: https://adventofcode.com/2020/day/19
[aoc_2020_18]: https://adventofcode.com/2020/day/18
