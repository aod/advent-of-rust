use std::collections::HashMap;

use super::{
    gate::Gate,
    instruction::Instruction,
    signal::{Signal, SignalProvider},
    source::{Source, WireIdentifier},
};

#[derive(Clone)]
pub struct Circuit(pub Vec<Instruction>);

impl Circuit {
    pub fn run(mut self) -> HashMap<WireIdentifier, Signal> {
        let mut wires = HashMap::new();
        while !self.0.is_empty() {
            self.0
                .retain(|instruction| !instruction.process(&mut wires).unwrap());
        }
        wires
    }
}

impl From<&str> for Circuit {
    fn from(input: &str) -> Self {
        let instrs = input
            .lines()
            .map(|line| {
                let mut tokens = line.split(' ').peekable();

                // Here we're parsing the left side of the instruction a.k.a the SignalProvider. For
                // example a Gate `x OR y`, a WireIdentifier `lx` or a number `123`.
                let signal = match tokens.next().unwrap() {
                    // Line looks like a NOT gate: `NOT x` -> y
                    "NOT" => SignalProvider::Gate(Gate::NOT(Source::parse(tokens.next().unwrap()))),
                    // We're `peeking` here since there is an edge case for `SignalProvider::Source -> WireIdentifier`. If we
                    // would have done `.next()` we would be at `->` and the code after `signal =
                    // match` block would panic.
                    sig => match *tokens.peek().unwrap() {
                        // Every arm of this match matches a binary operator, for example:
                        //    vvv____________________________^^^^^^^^^^^^^^^
                        // `x AND y -> z`.
                        //  ^^^^^^^___________vvvvv
                        // Now we are parsing gates which take 2 sources. `x` and `y` in the example
                        // above are potential sources for the gate.
                        "AND" => SignalProvider::Gate(Gate::AND(
                            Source::parse(sig),
                            // Skipping because of peek() in match.
                            Source::parse(tokens.nth(1).unwrap()),
                        )),
                        "OR" => SignalProvider::Gate(Gate::OR(
                            Source::parse(sig),
                            Source::parse(tokens.nth(1).unwrap()),
                        )),
                        "LSHIFT" => SignalProvider::Gate(Gate::LSHIFT(
                            Source::parse(sig),
                            Source::parse(tokens.nth(1).unwrap()),
                        )),
                        "RSHIFT" => SignalProvider::Gate(Gate::RSHIFT(
                            Source::parse(sig),
                            Source::parse(tokens.nth(1).unwrap()),
                        )),
                        // This is a SignalProvider which is a simple Source value: `123` -> x, or a
                        // wire identifier: `lx` -> y.
                        _ => SignalProvider::Source(Source::parse(sig)),
                    },
                };

                // We're here now.
                // vvv-------------or
                // 123 -> x        |
                //                 |
                // x AND y -> z    |
                //       ^---------or
                //                 |
                // NOT x -> y      |
                //     ^-----------or
                //
                // Now skip the `->` token and get the `output_wire`.
                //     ^^^^^^^^^^^^^_____vvvvvv
                let output_wire = tokens.nth(1).unwrap().to_string();

                Instruction {
                    signal,
                    output_wire,
                }
            })
            .collect();

        Self(instrs)
    }
}
