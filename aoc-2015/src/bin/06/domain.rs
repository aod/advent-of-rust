use std::iter::Sum;

use itertools::Itertools;

pub enum Phrase {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Coordinate {
    x: usize,
    y: usize,
}

struct Instruction {
    pub phrase: Phrase,
    pub rect: (Coordinate, Coordinate),
}

pub struct Instructions(Vec<Instruction>);

impl From<&str> for Instructions {
    fn from(input: &str) -> Self {
        let mut instructions = Vec::with_capacity(input.lines().count());

        for line in input.lines() {
            let mut it = line.split_whitespace();

            let phrase = match it.next().unwrap() {
                "toggle" => Phrase::Toggle,
                _ => match it.next().unwrap() {
                    "on" => Phrase::TurnOn,
                    "off" => Phrase::TurnOff,
                    _ => unreachable!(),
                },
            };

            let (x1, y1, x2, y2) = it
                .next()
                .unwrap()
                .split(',')
                .chain(it.nth(1).unwrap().split(','))
                .map(|v| v.parse().unwrap())
                .collect_tuple()
                .unwrap();

            instructions.push(Instruction {
                phrase,
                rect: (Coordinate { x: x1, y: y1 }, Coordinate { x: x2, y: y2 }),
            })
        }

        Self(instructions)
    }
}

impl Instructions {
    pub fn sum<Result, Cell, Callback>(self, f: Callback) -> Result
    where
        Result: Sum<Result> + From<Cell>,
        Cell: Default + Copy,
        Callback: Fn(&Phrase, &mut Cell),
    {
        const GRID_SIZE: usize = 1_000;
        let mut grid = vec![Cell::default(); GRID_SIZE * GRID_SIZE];

        for ins in self.0 {
            for y in ins.rect.0.y..=ins.rect.1.y {
                for x in ins.rect.0.x..=ins.rect.1.x {
                    f(&ins.phrase, &mut grid[y * GRID_SIZE + x]);
                }
            }
        }

        grid.iter().map(|t| Result::from(*t)).sum()
    }
}
