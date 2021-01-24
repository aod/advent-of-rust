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
        let mut keys = input.lines().map(|line| line.parse().unwrap());
        encryption_key(keys.next().unwrap(), loop_size(keys.next().unwrap()))
    }
}

impl Part2 for Day25 {
    type B = &'static str;

    fn solve(&self, _: &str) -> Self::B {
        "Merry Christmas!"
    }
}
