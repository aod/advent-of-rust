//! Advent of Code related concepts reside in this module.

use std::{fmt::Display, time::Instant};

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

/// A solver for both parts of an Advent of Code puzzle.
pub trait Solution: Part1 + Part2 {}
impl<T> Solution for T where T: Part1 + Part2 {}

/// Runs and prints the answer for the given solution.
///
/// This procedure will display the runtime duration and answer for both parts.
/// Answers are displayed on their own newline after the part header text with
/// the runtime timings.
pub fn solve_print<T, U>(sol: Box<dyn Solution<A = T, B = U>>, input: &str)
where
    T: Display,
    U: Display,
{
    {
        let now = Instant::now();
        let ans = Part1::solve(&*sol, input);
        let elapsed = now.elapsed();
        println!("Part1({:?}):\n{}", elapsed, ans);
    }

    {
        let now = Instant::now();
        let ans = Part2::solve(&*sol, input);
        let elapsed = now.elapsed();
        println!("Part2({:?}):\n{}", elapsed, ans);
    }
}
