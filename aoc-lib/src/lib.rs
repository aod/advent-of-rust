use std::{fmt::Display, time::Instant};

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
