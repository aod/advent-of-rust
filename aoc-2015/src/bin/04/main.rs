use std::fmt::Write;

use aoc_lib::{Part1, Part2, Solution};
use md5::{Digest, Md5};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/04.txt"));

fn main() {
    Day04::default().solve_print(INPUT);
}

#[derive(Default)]
struct Day04;

impl Part1 for Day04 {
    type A = usize;

    fn solve(&self, input: &str) -> Self::A {
        let mut secret = input.to_owned();
        let mut hasher = Md5::new();

        for i in 0..usize::MAX {
            write!(secret, "{}", i).expect("could not write to secret");
            hasher.update(&secret);

            let result = hasher.finalize_reset();
            if result[..2] == [0, 0] && result[2] <= 0x0F {
                return i;
            }

            secret.truncate(input.len());
        }

        unreachable!()
    }
}

impl Part2 for Day04 {
    type B = usize;

    fn solve(&self, input: &str) -> Self::B {
        let mut secret = input.to_owned();
        let mut hasher = Md5::new();

        for i in 0..usize::MAX {
            write!(secret, "{}", i).expect("could not write to secret");
            hasher.update(&secret);

            if hasher.finalize_reset()[..3] == [0, 0, 0] {
                return i;
            }

            secret.truncate(input.len());
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Day04, INPUT};
    use aoc_lib::{Part1, Part2};

    #[test]
    fn part1_example() {
        assert_eq!(Part1::solve(&Day04, "abcdef"), 609043);
        assert_eq!(Part1::solve(&Day04, "pqrstuv"), 1048970);
    }

    #[test]
    fn part1_answer() {
        assert_eq!(Part1::solve(&Day04, INPUT), 254575);
    }

    #[test]
    fn part2_answer() {
        assert_eq!(Part2::solve(&Day04, INPUT), 1038736);
    }
}
