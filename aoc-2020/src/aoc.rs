use std::fmt::Display;

pub trait Part1 {
    type A: Display;

    fn solve(&self, input: &str) -> Self::A;
}

pub trait Part2 {
    type B: Display;

    fn solve(&self, input: &str) -> Self::B;
}

pub trait Solution: Part1 + Part2 {}
impl<T> Solution for T where T: Part1 + Part2 {}
