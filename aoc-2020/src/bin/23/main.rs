mod domain;

use aoc_lib::{Part1, Part2, Solution};
use domain::{CrabCups, Cups};

const INPUT: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/23.txt"));

fn main() {
    Day23::default().solve_print(INPUT);
}

#[derive(Default)]
pub(crate) struct Day23 {
    move_amount: Option<usize>,
}

impl Part1 for Day23 {
    type A = String;

    fn solve(&self, input: &str) -> Self::A {
        let cups = Cups::from(input);
        let mut crab_cups = CrabCups::new(&cups);

        for _ in 0..self.move_amount.unwrap_or(100) {
            crab_cups.do_move();
        }

        crab_cups.label()
    }
}

impl Part2 for Day23 {
    type B = usize;

    fn solve(&self, input: &str) -> Self::B {
        let cups = Cups::from(input);
        let mut crab_cups = CrabCups::with_length(&cups, 1_000_000);

        for _ in 0..self.move_amount.unwrap_or(10_000_000) {
            crab_cups.do_move();
        }

        let a = crab_cups.next(1);
        let b = crab_cups.next(a);
        a * b
    }
}

#[cfg(test)]
mod tests {
    use super::{Day23, INPUT};
    use aoc_lib::{Part1, Part2};

    const EXAMPLE_INPUT: &'static str = "389125467";

    #[test]
    fn part1_example() {
        assert_eq!(
            Part1::solve(
                &Day23 {
                    move_amount: Some(10)
                },
                EXAMPLE_INPUT
            ),
            "92658374"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(Part2::solve(&Day23::default(), EXAMPLE_INPUT), 149245887792);
    }

    #[test]
    fn part1_answer() {
        assert_eq!(Part1::solve(&Day23::default(), INPUT), "24798635");
    }

    #[test]
    fn part2_answer() {
        assert_eq!(Part2::solve(&Day23::default(), INPUT), 12757828710);
    }
}
