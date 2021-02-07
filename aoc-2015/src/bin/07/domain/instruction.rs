use std::collections::HashMap;

use super::{
    signal::{Signal, SignalProvider},
    source::WireIdentifier,
};

/// Instruction represents a singe line of _instruction_ in the puzzle input. Each instruction has
/// a signal provider and an output wire e.g.: (`x AND y` -> `z`). See the
/// [SignalProvider](./enum.SignalProvider.html) enum for the possible _signals_.
#[derive(Clone)]
pub struct Instruction {
    pub signal: SignalProvider,
    pub output_wire: WireIdentifier,
}

impl Instruction {
    /// Processes the [SignalProvider](./enum.SignalProvider.html) and sets the wire signal to the
    /// output_wire. If the instruction could be processed it will return Some(true) otherwise
    /// Some(false). Returns none if wire already has a signal since this is not allowed by the
    /// puzzle instructions.
    pub(super) fn process(&self, wires: &mut HashMap<WireIdentifier, Signal>) -> Option<bool> {
        if wires.contains_key(&self.output_wire) {
            // A wire may only get a signal by 1 signal provider only.
            return None;
        }

        if let Some(signal) = match &self.signal {
            SignalProvider::Source(s) => s.signal(&wires),
            SignalProvider::Gate(gate) => gate.output(&wires),
        } {
            wires.insert(self.output_wire.clone(), signal);
            Some(true)
        } else {
            Some(false)
        }
    }
}
