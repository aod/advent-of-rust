mod domain;

use aoc_lib::{Part1, Part2, Solution};
use domain::{Instructions, Phrase};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/06.txt"));

fn main() {
    Day06::default().solve_print(INPUT);
}

#[derive(Default)]
struct Day06;

impl Part1 for Day06 {
    type A = usize;

    fn solve(&self, input: &str) -> Self::A {
        Instructions::from(input).sum(|phrase, lit: &mut bool| {
            *lit = match phrase {
                Phrase::TurnOn => true,
                Phrase::TurnOff => false,
                Phrase::Toggle => !*lit,
            }
        })
    }
}

impl Part2 for Day06 {
    type B = usize;

    fn solve(&self, input: &str) -> Self::B {
        Instructions::from(input).sum(|phrase, brightness: &mut usize| match phrase {
            Phrase::TurnOn => *brightness += 1,
            Phrase::TurnOff => *brightness = brightness.checked_sub(1).unwrap_or_else(|| 0),
            Phrase::Toggle => *brightness += 2,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Day06, INPUT};
    use aoc_lib::{Part1, Part2};

    #[test]
    fn part1_example() {
        assert_eq!(
            Part1::solve(&Day06, "turn on 0,0 through 999,999"),
            1_000 * 1_000
        );
        assert_eq!(
            Part1::solve(&Day06, "toggle 0,0 through 999,999"),
            1_000 * 1_000
        );
        assert_eq!(Part1::solve(&Day06, "turn on 499,499 through 500,500"), 4);
    }

    #[test]
    fn part2_example() {
        assert_eq!(Part2::solve(&Day06, "turn on 0,0 through 0,0"), 1);
        assert_eq!(
            Part2::solve(&Day06, "toggle 0,0 through 999,999"),
            2_000_000
        );
    }

    #[test]
    fn part1_answer() {
        assert_eq!(Part1::solve(&Day06::default(), INPUT), 400410);
    }

    #[test]
    fn part2_answer() {
        assert_eq!(Part2::solve(&Day06::default(), INPUT), 15343601);
    }
}
