use super::{gate::Gate, source::Source};

pub type Signal = u16;

/// A SignalProvider is simply a [Source](./enum.Source.html) which is a wire or an u16 value, or a
/// [Gate](./enum.Gate.html) which uses Source's to do bitwise operation.
#[derive(Clone)]
pub enum SignalProvider {
    Source(Source),
    Gate(Gate),
}
