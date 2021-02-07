use aoc_lib::{Part1, Part2, Solution};
use itertools::Itertools;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/02.txt"));

fn main() {
    Day02::default().solve_print(INPUT);
}

#[derive(Default)]
struct Day02;

impl Part1 for Day02 {
    type A = usize;

    fn solve(&self, input: &str) -> Self::A {
        input.lines().fold(0, |sum, line| {
            let (l, w, h) = line
                .split('x')
                .map(|v| v.parse().unwrap())
                .collect_tuple::<(usize, usize, usize)>()
                .expect("expected 3 elements");

            let lw = l * w;
            let wh = w * h;
            let hl = h * l;
            sum + 2 * lw + 2 * wh + 2 * hl + [lw, wh, hl].iter().min().unwrap()
        })
    }
}

impl Part2 for Day02 {
    type B = usize;

    fn solve(&self, input: &str) -> Self::B {
        input.lines().fold(0, |sum, line| {
            let (l, w, h) = line
                .split('x')
                .map(|v| v.parse().unwrap())
                .collect_tuple::<(usize, usize, usize)>()
                .expect("Expected 3 elements");

            let lw = l * 2 + w * 2;
            let wh = w * 2 + h * 2;
            let hl = h * 2 + l * 2;
            sum + w * h * l + [lw, wh, hl].iter().min().unwrap()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Day02, INPUT};
    use aoc_lib::{Part1, Part2};

    #[test]
    fn part1_example() {
        assert_eq!(Part1::solve(&Day02, "2x3x4"), 58);
        assert_eq!(Part1::solve(&Day02, "1x1x10"), 43);
    }

    #[test]
    fn part2_example() {
        assert_eq!(Part2::solve(&Day02, "2x3x4"), 34);
        assert_eq!(Part2::solve(&Day02, "1x1x10"), 14);
    }

    #[test]
    fn part1_answer() {
        assert_eq!(Part1::solve(&Day02, INPUT), 1588178);
    }

    #[test]
    fn part2_answer() {
        assert_eq!(Part2::solve(&Day02, INPUT), 3783758);
    }
}
