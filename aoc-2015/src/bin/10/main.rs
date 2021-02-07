mod domain;

use aoc_lib::{Part1, Part2, Solution};
use domain::LookAndSay;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/10.txt"));

fn main() {
    Day10::default().solve_print(INPUT);
}

#[derive(Default)]
struct Day10;

impl Part1 for Day10 {
    type A = usize;

    fn solve(&self, input: &str) -> Self::A {
        LookAndSay::new(input)
            .nth(39)
            .map(|sequence| sequence.len())
            .unwrap()
    }
}

impl Part2 for Day10 {
    type B = usize;

    fn solve(&self, input: &str) -> Self::B {
        LookAndSay::new(input)
            .nth(49)
            .map(|sequence| sequence.len())
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::{Day10, INPUT};
    use aoc_lib::{Part1, Part2};

    #[test]
    fn part1_answer() {
        assert_eq!(Part1::solve(&Day10, INPUT), 360154);
    }

    #[test]
    fn part2_answer() {
        assert_eq!(Part2::solve(&Day10, INPUT), 5103798);
    }
}
