//! Advent of Code related concepts reside in this module.

use std::{
    fmt::Display,
    time::{Duration, Instant},
};

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
pub trait Solution: Part1 + Part2 {
    /// Runs and prints the answer for the given solution.
    ///
    /// This procedure will display the runtime duration and answer for both parts.
    /// Answers are displayed on their own newline after the part header text with
    /// the runtime timings.
    fn solve_print(&self, input: &str)
    where
        Self: Sized,
    {
        (self as &dyn Part1<A = Self::A>).solve_print(input, "Part1");
        (self as &dyn Part2<B = Self::B>).solve_print(input, "Part2");
    }
}
impl<T: Part1 + Part2> Solution for T {}

pub trait Solver {
    type Answer: Display;

    fn solve(&self, input: &str) -> Self::Answer;

    fn solve_print(&self, input: &str, header: &str) {
        let (ans, elapsed) = self.time(input);
        println!("{}({:?})\n{}", header, elapsed, ans)
    }

    fn time(&self, input: &str) -> (Self::Answer, Duration) {
        let now = Instant::now();
        let ans = self.solve(input);
        let elapsed = now.elapsed();
        (ans, elapsed)
    }
}

impl<T: Display> Solver for &dyn Part1<A = T> {
    type Answer = T;

    fn solve(&self, input: &str) -> Self::Answer {
        Part1::solve(*self, input)
    }
}

impl<T: Display> Solver for &dyn Part2<B = T> {
    type Answer = T;

    fn solve(&self, input: &str) -> Self::Answer {
        Part2::solve(*self, input)
    }
}
