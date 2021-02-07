use aoc_lib::{Part1, Part2, Solution};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/01.txt"));

fn main() {
    Day01::default().solve_print(INPUT);
}

#[derive(Default)]
struct Day01;

impl Part1 for Day01 {
    type A = isize;

    fn solve(&self, input: &str) -> Self::A {
        input
            .chars()
            .fold(0, |floor, instruction| match instruction {
                '(' => floor + 1,
                ')' => floor - 1,
                _ => unreachable!(),
            })
    }
}

impl Part2 for Day01 {
    type B = usize;

    fn solve(&self, input: &str) -> Self::B {
        let mut it = input.chars();
        it.try_fold(0usize, |floor, instruction| match instruction {
            '(' => Some(floor + 1),
            ')' => floor.checked_sub(1),
            _ => unreachable!(),
        });
        input.len() - it.count()
    }
}

#[cfg(test)]
mod tests {
    use super::{Day01, INPUT};
    use aoc_lib::{Part1, Part2};

    #[test]
    fn part1_example() {
        assert_eq!(Part1::solve(&Day01, "(())"), 0);
        assert_eq!(Part1::solve(&Day01, "()()"), 0);

        assert_eq!(Part1::solve(&Day01, "((("), 3);
        assert_eq!(Part1::solve(&Day01, "(()(()("), 3);

        assert_eq!(Part1::solve(&Day01, "))((((("), 3);

        assert_eq!(Part1::solve(&Day01, "())"), -1);
        assert_eq!(Part1::solve(&Day01, "))("), -1);

        assert_eq!(Part1::solve(&Day01, ")))"), -3);
        assert_eq!(Part1::solve(&Day01, ")())())"), -3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(Part2::solve(&Day01, ")"), 1);
        assert_eq!(Part2::solve(&Day01, "()())"), 5);
    }

    #[test]
    fn part1_answer() {
        assert_eq!(Part1::solve(&Day01, INPUT), 138);
    }

    #[test]
    fn part2_answer() {
        assert_eq!(Part2::solve(&Day01, INPUT), 1771);
    }
}
