use std::collections::HashMap;

use super::{
    signal::Signal,
    source::{Source, WireIdentifier},
};

/// A Gate is a bitwise operation on 1 on or more [Source](./enum.Source.html)'s. Possible bitwise
/// operations include: AND, OR, LSHIFT, RSHIFT or NOT.
#[derive(Clone, Debug)]
pub enum Gate {
    AND(Source, Source),
    OR(Source, Source),
    LSHIFT(Source, Source),
    RSHIFT(Source, Source),
    NOT(Source),
}

impl Gate {
    /// Calculates and returns the gate's output only if all sources are "valid".  A valid
    /// [Source](./enum.Source.html) is an `u16` value or a wire that `exists` in the given `wires`
    /// HashMap.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use]
    /// # use aoc_2015::day07::{Gate, Source};
    /// # use std::collections::HashMap;
    /// # fn main() {
    /// let wires = [("x".into(), 1)].iter().cloned().collect();
    ///
    /// assert_eq!(Gate::AND(Source::Value(1), Source::Value(1)).output(&wires),
    ///            Some(1));
    /// assert_eq!(Gate::AND(Source::Value(1), Source::Wire("x".to_string())).output(&wires),
    ///            Some(1));
    /// // The wire `y` does not exist _____________________vvv
    /// assert_eq!(Gate::AND(Source::Value(1), Source::Wire("y".to_string())).output(&wires),
    ///            None);
    /// # }
    /// ```
    pub(super) fn output(&self, wires: &HashMap<WireIdentifier, Signal>) -> Option<Signal> {
        match &self {
            Gate::AND(s1, s2) => s1
                .signal(&wires)
                .and_then(|v1| s2.signal(&wires).map(|v2| v1 & v2)),
            Gate::OR(s1, s2) => s1
                .signal(&wires)
                .and_then(|v1| s2.signal(&wires).map(|v2| v1 | v2)),
            Gate::LSHIFT(s1, s2) => s1
                .signal(&wires)
                .and_then(|v1| s2.signal(&wires).map(|v2| v1 << v2)),
            Gate::RSHIFT(s1, s2) => s1
                .signal(&wires)
                .and_then(|v1| s2.signal(&wires).map(|v2| v1 >> v2)),
            Gate::NOT(s1) => s1.signal(&wires).map(|v1| !v1),
        }
    }
}
