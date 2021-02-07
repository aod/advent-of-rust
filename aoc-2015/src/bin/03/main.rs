use std::{cell::Cell, collections::HashSet};

use aoc_lib::{Part1, Part2, Solution};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/03.txt"));

fn main() {
    Day03::default().solve_print(INPUT);
}

#[derive(Default)]
struct Day03;

impl Part1 for Day03 {
    type A = usize;

    fn solve(&self, input: &str) -> Self::A {
        let mut houses = HashSet::with_capacity(input.len());
        houses.insert((0, 0));

        let mut santa = (0, 0);
        for move_xy in input.chars() {
            match move_xy {
                '^' => santa.1 += 1,
                'v' => santa.1 -= 1,
                '>' => santa.0 += 1,
                '<' => santa.0 -= 1,
                _ => unreachable!(),
            };
            houses.insert(santa);
        }

        houses.len()
    }
}

impl Part2 for Day03 {
    type B = usize;

    fn solve(&self, input: &str) -> Self::B {
        let mut houses = HashSet::with_capacity(input.len());
        houses.insert((0, 0));

        let santas = vec![Cell::new((0, 0)); 2];
        for (santa, direction) in santas.iter().cycle().zip(input.chars()) {
            let mut current_santa = santa.get();
            match direction {
                '^' => current_santa.1 += 1,
                'v' => current_santa.1 -= 1,
                '>' => current_santa.0 += 1,
                '<' => current_santa.0 -= 1,
                _ => unreachable!(),
            };
            houses.insert(current_santa);
            santa.set(current_santa);
        }

        houses.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{Day03, INPUT};
    use aoc_lib::{Part1, Part2};

    #[test]
    fn part1_example() {
        assert_eq!(Part1::solve(&Day03, ">"), 2);
        assert_eq!(Part1::solve(&Day03, "^>v<"), 4);
        assert_eq!(Part1::solve(&Day03, "^v^v^v^v^v"), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(Part2::solve(&Day03, ">"), 2);
        assert_eq!(Part2::solve(&Day03, "^>v<"), 3);
        assert_eq!(Part2::solve(&Day03, "^v^v^v^v^v"), 11);
    }

    #[test]
    fn part1_answer() {
        assert_eq!(Part1::solve(&Day03, INPUT), 2565);
    }

    #[test]
    fn part2_answer() {
        assert_eq!(Part2::solve(&Day03, INPUT), 2639);
    }
}
