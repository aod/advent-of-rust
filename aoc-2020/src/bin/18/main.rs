mod domain;

use aoc_lib::{Part1, Part2, Solution};
use domain::eval;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/18.txt"));

fn main() {
    Day18::default().solve_print(INPUT);
}

#[derive(Default)]
struct Day18;

impl Part1 for Day18 {
    type A = u64;

    fn solve(&self, input: &str) -> Self::A {
        input.lines().map(eval).sum()
    }
}

impl Part2 for Day18 {
    type B = u64;

    fn solve(&self, _input: &str) -> Self::B {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::{Day18, INPUT};
    use aoc_lib::Part1;

    #[test]
    #[rustfmt::skip]
    fn part1_example() {
        assert_eq!(Part1::solve(&Day18::default(), 
            "2 * 3 + (4 * 5)"), 26);
        assert_eq!(Part1::solve(&Day18::default(), 
            "5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(Part1::solve(&Day18::default(), 
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(Part1::solve(&Day18::default(), 
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
    }

    #[test]
    fn part1_answer() {
        assert_eq!(Part1::solve(&Day18::default(), INPUT), 36382392389406);
    }
}
