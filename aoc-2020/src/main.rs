mod aoc;
mod day21;

use std::{env, fmt::Display, fs, process};

use aoc::{Part1, Part2, Solution};
use day21::Day21;

fn main() {
    let mut args = env::args().skip(1);
    match args.next() {
        Some(solver) => {
            let file_path = match args.next() {
                Some(path) => path,
                None => {
                    eprintln!("Invalid input file path");
                    process::exit(1);
                }
            };

            let input = match fs::read_to_string(file_path) {
                Ok(contents) => contents,
                Err(err) => {
                    eprintln!("Could not read file: {}", err);
                    process::exit(1);
                }
            };

            match solver.as_str() {
                "21" => solve_print(Box::new(Day21::default()), &input),
                _ => {
                    eprintln!("Invalid solver `{}`", solver);
                    process::exit(1);
                }
            }
        }
        None => {
            eprintln!("No solver was provided");
            process::exit(1);
        }
    }
}

fn solve_print<T, U>(sol: Box<dyn Solution<A = T, B = U>>, input: &str)
where
    T: Display,
    U: Display,
{
    println!(
        "Part 1:\n\
         {}\n\
         Part 2:\n\
         {}",
        Part1::solve(&*sol, input),
        Part2::solve(&*sol, input)
    );
}
