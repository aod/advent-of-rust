mod domain;

use aoc_lib::{Part1, Part2, Solution};
use domain::{circuit::Circuit, signal::SignalProvider, source::Source};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/07.txt"));

fn main() {
    Day07::default().solve_print(INPUT);
}

#[derive(Default)]
struct Day07;

impl Part1 for Day07 {
    type A = u16;

    fn solve(&self, input: &str) -> Self::A {
        *Circuit::from(input)
            .run()
            .get("a")
            .expect("wire a does not exist")
    }
}

impl Part2 for Day07 {
    type B = u16;

    fn solve(&self, input: &str) -> Self::B {
        let mut circuit = Circuit::from(input);
        let signal_a = *circuit.clone().run().get("a").unwrap();

        circuit
            .0
            .iter_mut()
            .find(|x| x.output_wire == "b")
            .expect("could not find wire b")
            .signal = SignalProvider::Source(Source::Value(signal_a));

        *circuit.run().get("a").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::circuit::Circuit;

    use super::{Day07, INPUT};
    use aoc_lib::{Part1, Part2};

    #[test]
    fn part1_example() {
        let simple_circuit = "\
            123 -> x\n\
            456 -> y\n\
            x AND y -> d\n\
            x OR y -> e\n\
            x LSHIFT 2 -> f\n\
            y RSHIFT 2 -> g\n\
            NOT x -> h\n\
            NOT y -> i";

        assert_eq!(
            Circuit::from(simple_circuit).run(),
            [
                ("d".into(), 72),
                ("e".into(), 507),
                ("f".into(), 492),
                ("g".into(), 114),
                ("h".into(), 65412),
                ("i".into(), 65079),
                ("x".into(), 123),
                ("y".into(), 456),
            ]
            .iter()
            .cloned()
            .collect()
        );
    }

    #[test]
    fn part1_answer() {
        assert_eq!(Part1::solve(&Day07, INPUT), 46065);
    }

    #[test]
    fn part2_answer() {
        assert_eq!(Part2::solve(&Day07, INPUT), 14134);
    }
}
