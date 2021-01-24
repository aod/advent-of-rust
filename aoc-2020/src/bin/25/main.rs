mod domain;

use aoc_lib::{Part1, Part2, Solution};
use domain::{encryption_key, loop_size};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/25.txt"));

fn main() {
    Day25::default().solve_print(INPUT);
}

#[derive(Default)]
struct Day25;

impl Part1 for Day25 {
    type A = usize;

    fn solve(&self, input: &str) -> Self::A {
        let mut keys = input.lines().map(str::parse).flatten();
        encryption_key(keys.next().unwrap(), loop_size(keys.next().unwrap()))
    }
}

impl Part2 for Day25 {
    type B = &'static str;

    fn solve(&self, _: &str) -> Self::B {
        "Merry Christmas!"
    }
}

#[cfg(test)]
mod tests {
    use crate::{Day25, INPUT};
    use aoc_lib::Part1;

    #[test]
    fn part1_example() {
        assert_eq!(Part1::solve(&Day25::default(), EXAMPLE_INPUT), 14897079);
    }

    #[test]
    fn part1_answer() {
        assert_eq!(Part1::solve(&Day25::default(), INPUT), 4441893);
    }

    const EXAMPLE_INPUT: &str = "\
        5764801\n\
        17807724";
}
