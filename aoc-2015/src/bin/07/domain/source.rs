use std::collections::HashMap;

use super::signal::Signal;

pub type WireIdentifier = String;

/// A source is a **wire identifier** or an **`u16` signal**.
#[derive(Clone, Debug)]
pub enum Source {
    Wire(WireIdentifier),
    Value(Signal),
}

impl Source {
    /// Returns the signal of a Source.
    pub(super) fn signal(&self, wires: &HashMap<WireIdentifier, Signal>) -> Option<Signal> {
        match &self {
            Source::Value(signal) => Some(*signal),
            Source::Wire(wire) => wires.get(wire).copied(),
        }
    }

    /// A helper function that tries to parse the given string to a Source::Value. If this does not
    /// succeed it is assumed that the given string is a wire identifier and Source::Wire is
    /// returned.
    pub(super) fn parse(wire_or_value: &str) -> Source {
        wire_or_value
            .parse()
            .map(Source::Value)
            .unwrap_or_else(|_| Source::Wire(wire_or_value.to_string()))
    }
}
