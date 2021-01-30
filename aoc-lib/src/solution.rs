use crate::{Part1, Part2, Solver};

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
