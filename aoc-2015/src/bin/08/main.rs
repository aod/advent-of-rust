use aoc_lib::{Part1, Part2, Solution};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/08.txt"));

fn main() {
    Day08::default().solve_print(INPUT);
}

#[derive(Default)]
struct Day08;

impl Part1 for Day08 {
    type A = usize;

    fn solve(&self, input: &str) -> Self::A {
        input
            .lines()
            .map(|line| line[1..line.len() - 1].chars())
            .try_fold(0, |sum, mut it| {
                let mut line_sum = 2;
                while it.any(|c| c == '\\') {
                    line_sum += match it.next()? {
                        'x' => 3,
                        _ => 1,
                    }
                }
                Some(sum + line_sum)
            })
            .unwrap()
    }
}

impl Part2 for Day08 {
    type B = usize;

    fn solve(&self, input: &str) -> Self::B {
        input.lines().fold(0, |sum, line| {
            sum + line.escape_default().count() - line.len() + 2
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Day08, INPUT};
    use aoc_lib::{Part1, Part2};

    #[test]
    fn part1_example() {
        assert_eq!(Part1::solve(&Day08, r#""""#), 2);
        assert_eq!(Part1::solve(&Day08, r#""abc""#), 2);
        assert_eq!(Part1::solve(&Day08, r#""aaa\"aaa""#), 3);
        assert_eq!(Part1::solve(&Day08, r#""\x27""#), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(Part2::solve(&Day08, r#""""#), 4);
        assert_eq!(Part2::solve(&Day08, r#""abc""#), 4);
        assert_eq!(Part2::solve(&Day08, r#""aaa\"aaa""#), 6);
        assert_eq!(Part2::solve(&Day08, r#""\x27""#), 5);
    }

    #[test]
    fn part1_answer() {
        assert_eq!(Part1::solve(&Day08, INPUT), 1371);
    }

    #[test]
    fn part2_answer() {
        assert_eq!(Part2::solve(&Day08, INPUT), 2117);
    }
}
