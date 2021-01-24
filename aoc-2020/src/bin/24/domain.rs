use std::{collections::HashSet, ops::Add};

/// Because the tiles are hexagonal, every tile has six neighbors: east,
/// southeast, southwest, west, northwest, and northeast. These directions are
/// given in your list, respectively, as `e`, `se`, `sw`, `w`, `nw`, and `ne`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

impl Direction {
    #[rustfmt::skip]
    fn delta(&self) -> Point {
        match self {
            Self::E  => Point { x:  1, y: -1, z:  0 },
            Self::SE => Point { x:  0, y: -1, z:  1 },
            Self::SW => Point { x: -1, y:  0, z:  1 },
            Self::W  => Point { x: -1, y:  1, z:  0 },
            Self::NW => Point { x:  0, y:  1, z: -1 },
            Self::NE => Point { x:  1, y:  0, z: -1 },
        }
    }
}

/// A tile is identified by a series of directions.
#[derive(Debug, Default)]
pub(crate) struct Tile(Vec<Direction>);

impl From<&str> for Tile {
    fn from(input: &str) -> Self {
        let mut tile: Tile = Default::default();
        let mut chars = input.chars();

        while let Some(dir) = chars.next() {
            let dir = match dir {
                'e' => Direction::E,
                'w' => Direction::W,
                's' => {
                    if chars.next().unwrap() == 'e' {
                        Direction::SE
                    } else {
                        Direction::SW
                    }
                }
                'n' => {
                    if chars.next().unwrap() == 'e' {
                        Direction::NE
                    } else {
                        Direction::NW
                    }
                }
                _ => unreachable!(),
            };
            tile.0.push(dir);
        }

        tile
    }
}

/// Each element in the list identifies a single tile that needs to be flipped
/// by giving a series of steps starting from a reference tile in the very
/// center of the room (0, 0, 0). (Every line starts from the same reference
/// tile.)
#[derive(Debug)]
pub(crate) struct Tiles(Vec<Tile>);

impl From<&str> for Tiles {
    fn from(input: &str) -> Self {
        Self(input.lines().map(From::from).collect())
    }
}

#[derive(Debug, Clone, Default, Copy, Hash, PartialEq, Eq)]
pub(crate) struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Point {
    fn delta_nbors(&self) -> impl Iterator<Item = Point> + '_ {
        use Direction::*;
        [E, SE, SW, W, NW, NE]
            .iter()
            .map(|direction| direction.delta())
            .map(move |point| *self + point)
    }
}

#[derive(Debug, Default, Clone)]
pub(crate) struct BlackTiles(pub(crate) HashSet<Point>);

impl BlackTiles {
    fn black_nbors(&self, point: &Point) -> usize {
        point
            .delta_nbors()
            .filter(|point| self.0.contains(point))
            .count()
    }

    pub(crate) fn flip_tiles(lines: &Tiles) -> Self {
        let mut result: Self = Default::default();

        for steps in &lines.0 {
            let point: Point = steps
                .0
                .iter()
                .map(Direction::delta)
                .fold(Default::default(), Add::add);

            if !result.0.insert(point) {
                result.0.remove(&point);
            }
        }

        result
    }
}

impl Iterator for BlackTiles {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next: Self = Default::default();
        let candidates = self
            .0
            .iter()
            .flat_map(Point::delta_nbors)
            .chain(self.0.iter().copied());

        for candidate in candidates {
            let nbors = self.black_nbors(&candidate);
            let is_black_tile = self.0.contains(&candidate);

            if (is_black_tile && (nbors == 1 || nbors == 2)) || (!is_black_tile && nbors == 2) {
                next.0.insert(candidate);
            }
        }

        *self = next.clone();
        Some(next)
    }
}
