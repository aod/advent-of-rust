use std::fmt::Display;

/// A solver for part 1 of an Advent of Code puzzle.
pub trait Part1 {
    /// The answer for part 1 which must be able to in someway display itself to
    /// the user.
    type A: Display;

    /// The solve method will parse, process and return the answer for a part of
    /// a puzzle.
    ///
    /// For any given year and day of an Advent of Code puzzle the implementer
    /// may assume the input is always valid, as are the original and example
    /// inputs on the official website.
    fn solve(&self, input: &str) -> Self::A;
}

/// A solver for part 2 of an Advent of Code puzzle.
///
/// Pretty much the same thing as [Part1](Part1) except representing
/// part 2 of a puzzle... See [Part1](Part1) for more info.
pub trait Part2 {
    type B: Display;

    fn solve(&self, input: &str) -> Self::B;
}
