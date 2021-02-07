mod domain;

use aoc_lib::{Part1, Part2, Solution};
use domain::XMAS;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/9.txt"));

fn main() {
    Day09::default().solve_print(INPUT);
}

pub struct Day09 {
    pub preamble_count: usize,
}

impl Default for Day09 {
    fn default() -> Self {
        Self { preamble_count: 25 }
    }
}

impl Part1 for Day09 {
    type A = usize;

    fn solve(&self, input: &str) -> Self::A {
        let numbers: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();

        let preamble = &numbers[..self.preamble_count];
        let mut cypher = XMAS::new(preamble);

        for x in &numbers[self.preamble_count..] {
            let is_valid = cypher.next(*x);
            if !is_valid {
                return *x;
            }
        }

        unreachable!()
    }
}

impl Part2 for Day09 {
    type B = usize;

    fn solve(&self, input: &str) -> Self::B {
        let parsed: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
        let target = Part1::solve(self, input);

        for (start_idx, val) in parsed.iter().enumerate() {
            let mut processed = 0;
            let mut sum = *val;

            let mut xs = parsed[start_idx + 1..].iter();
            while sum < target {
                match xs.next() {
                    Some(v) => {
                        sum += v;
                        processed += 1;
                    }
                    None => break,
                }
            }

            if processed >= 2 && sum == target {
                let range = &parsed[start_idx..=start_idx + processed];
                let min = range.iter().min().expect("could not get min");
                let max = range.iter().max().expect("could not get max");
                return min + max;
            }
        }

        unreachable!();
    }
}

// NOTE: Example tests are located in domain.rs
#[cfg(test)]
mod tests {
    use super::{Day09, INPUT};
    use aoc_lib::{Part1, Part2};

    #[test]
    fn part1_answer() {
        assert_eq!(Part1::solve(&Day09::default(), INPUT), 21806024);
    }

    #[test]
    fn part2_answer() {
        assert_eq!(Part2::solve(&Day09::default(), INPUT), 2986195);
    }
}
