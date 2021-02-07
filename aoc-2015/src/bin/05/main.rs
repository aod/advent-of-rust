use aoc_lib::{Part1, Part2, Solution};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/05.txt"));

fn main() {
    Day05::default().solve_print(INPUT);
}

#[derive(Default)]
struct Day05;

impl Part1 for Day05 {
    type A = usize;

    fn solve(&self, input: &str) -> Self::A {
        const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];
        const BAD_STRS: &[&str] = &["ab", "cd", "pq", "xy"];

        input.lines().fold(0, |nice, line| {
            if line.split(VOWELS).count() > 3
                && !BAD_STRS.iter().any(|bad_str| line.contains(bad_str))
                && line.as_bytes().windows(2).any(|pair| pair[0] == pair[1])
            {
                nice + 1
            } else {
                nice
            }
        })
    }
}

impl Part2 for Day05 {
    type B = usize;

    fn solve(&self, input: &str) -> Self::B {
        input.lines().fold(0, |nice, line| {
            let is_nice_string = line.as_bytes().windows(2).enumerate().any(|(i, pair)| {
                line.rfind(std::str::from_utf8(pair).unwrap()) // Find the last occurrence of the current pair
                    .map(|index| index > i + 1) // Resulting pair must not overlap
                    .unwrap_or(false)
            }) && line.as_bytes().windows(3).any(|pair| pair[0] == pair[2]);

            if is_nice_string {
                nice + 1
            } else {
                nice
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Day05, INPUT};
    use aoc_lib::{Part1, Part2};

    #[test]
    fn part1_example() {
        assert_eq!(Part1::solve(&Day05, "ugknbfddgicrmopn"), 1);
        assert_eq!(Part1::solve(&Day05, "aaa"), 1);
        assert_eq!(Part1::solve(&Day05, "jchzalrnumimnmhp"), 0);
        assert_eq!(Part1::solve(&Day05, "haegwjzuvuyypxyu"), 0);
        assert_eq!(Part1::solve(&Day05, "dvszwmarrgswjxmb"), 0);
    }

    #[test]
    fn part2_example() {
        assert_eq!(Part2::solve(&Day05, "qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(Part2::solve(&Day05, "xxyxx"), 1);
        assert_eq!(Part2::solve(&Day05, "uurcxstgmygtbstg"), 0);
        assert_eq!(Part2::solve(&Day05, "ieodomkazucvgmuy"), 0);
    }

    #[test]
    fn part1_answer() {
        assert_eq!(Part1::solve(&Day05, INPUT), 238);
    }

    #[test]
    fn part2_answer() {
        assert_eq!(Part2::solve(&Day05, INPUT), 69);
    }
}
