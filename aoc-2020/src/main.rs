mod day21;
mod day22;
mod day23;

use std::{env, fs, process};

use aoc_lib::solve_print;
use day21::Day21;
use day22::Day22;
use day23::Day23;

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
                "22" => solve_print(Box::new(Day22::default()), &input),
                "23" => solve_print(Box::new(Day23::default()), &input),
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
