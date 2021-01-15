mod domain;

use aoc_lib::{Part1, Part2};

use self::domain::Game;

#[derive(Default)]
pub(crate) struct Day22;

impl Part1 for Day22 {
    type A = usize;

    fn solve(&self, input: &str) -> Self::A {
        Game::from(input).simulate_combat().score()
    }
}

impl Part2 for Day22 {
    type B = usize;

    fn solve(&self, input: &str) -> Self::B {
        Game::from(input).simulate_recursive_combat().score()
    }
}

#[cfg(test)]
mod tests {
    use super::Day22;
    use crate::aoc::{Part1, Part2};

    const EXAMPLE_INPUT: &'static str = "\
        Player 1:\n\
        9\n\
        2\n\
        6\n\
        3\n\
        1\n\n\

        Player 2:\n\
        5\n\
        8\n\
        4\n\
        7\n\
        10";

    #[test]
    fn part1_example() {
        assert_eq!(Part1::solve(&Day22::default(), EXAMPLE_INPUT), 306);
    }

    #[test]
    fn part2_example() {
        assert_eq!(Part2::solve(&Day22::default(), EXAMPLE_INPUT), 291);
    }

    const INPUT: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/22.txt"));

    #[test]
    fn part1_answer() {
        assert_eq!(Part1::solve(&Day22::default(), INPUT), 33010);
    }

    #[test]
    fn part2_answer() {
        assert_eq!(Part2::solve(&Day22::default(), INPUT), 32769);
    }
}
