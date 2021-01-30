use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use crate::{Part1, Part2};

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
