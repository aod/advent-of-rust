mod domain;

use aoc_lib::{Part1, Part2, Solution};
use domain::RouteMap;
use itertools::MinMaxResult;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/09.txt"));

fn main() {
    Day08::default().solve_print(INPUT);
}

#[derive(Default)]
struct Day08;

impl Part1 for Day08 {
    type A = usize;

    fn solve(&self, input: &str) -> Self::A {
        RouteMap::from(input.lines())
            .0
            .into_iter()
            .collect::<MinMaxResult<_>>()
            .into_option()
            .map(|mm| mm.0)
            .expect("could not find shortest route")
    }
}

impl Part2 for Day08 {
    type B = usize;

    fn solve(&self, input: &str) -> Self::B {
        RouteMap::from(input.lines())
            .0
            .into_iter()
            .collect::<MinMaxResult<_>>()
            .into_option()
            .map(|mm| mm.1)
            .expect("could not find longest route")
    }
}

#[cfg(test)]
mod tests {
    use super::{Day08, INPUT};
    use aoc_lib::{Part1, Part2};

    const EXAMPLE_INPUT: &str = "\
        London to Dublin = 464\n\
        London to Belfast = 518\n\
        Dublin to Belfast = 141";

    #[test]
    fn part1_example() {
        assert_eq!(Part1::solve(&Day08, EXAMPLE_INPUT), 605);
    }

    #[test]
    fn part2_example() {
        assert_eq!(Part2::solve(&Day08, EXAMPLE_INPUT), 982);
    }

    #[test]
    fn part1_answer() {
        assert_eq!(Part1::solve(&Day08, INPUT), 117);
    }

    #[test]
    fn part2_answer() {
        assert_eq!(Part2::solve(&Day08, INPUT), 909);
    }
}
