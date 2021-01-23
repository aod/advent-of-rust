use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

use super::{
    direction::{CardinalDir, OrdinalDir},
    orient::{Orientable, Orientations},
};

pub(crate) const TILE_SIZE: usize = 10;
pub(crate) const X: TileCell = TileCell(true);
pub(crate) const O: TileCell = TileCell(false);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// A single cell represented in a Tile.
///
/// true = `#`, false = `.`
pub(crate) struct TileCell(pub(crate) bool);

impl std::fmt::Display for TileCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 {
            Ok(write!(f, "#"))?
        } else {
            Ok(write!(f, "."))?
        }
    }
}

/// Tile - A small image from camera array
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Tile {
    /// Unieuqe identifier of the tile
    pub(crate) id: usize,
    /// The image chunk data
    pub(crate) data: [[TileCell; TILE_SIZE]; TILE_SIZE],
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Tile {}

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Orientable for Tile {
    fn flip(&mut self) {
        self.data.reverse()
    }

    fn rotate(&mut self) {
        let mut rotated: [[TileCell; TILE_SIZE]; TILE_SIZE] = Default::default();
        for i in 0..TILE_SIZE {
            for j in 0..TILE_SIZE {
                rotated[i][j] = self.data[TILE_SIZE - j - 1][i];
            }
        }

        self.data = rotated;
    }
}

impl Tile {
    pub(crate) fn top_border(&self) -> impl Iterator<Item = &TileCell> {
        self.data[0].iter()
    }

    pub(crate) fn bottom_border(&self) -> impl Iterator<Item = &TileCell> {
        self.data[TILE_SIZE - 1].iter()
    }

    pub(crate) fn right_border(&self) -> impl Iterator<Item = &TileCell> {
        self.data.iter().map(|row| &row[TILE_SIZE - 1])
    }

    pub(crate) fn left_border(&self) -> impl Iterator<Item = &TileCell> {
        self.data.iter().map(|row| &row[0])
    }

    pub(crate) fn stitch_to(&self, other: &Self) -> Option<CardinalDir> {
        use CardinalDir::*;

        if self.top_border().eq(other.bottom_border()) {
            Some(North)
        } else if self.right_border().eq(other.left_border()) {
            Some(East)
        } else if self.bottom_border().eq(other.top_border()) {
            Some(South)
        } else if self.left_border().eq(other.right_border()) {
            Some(West)
        } else {
            None
        }
    }

    pub(super) fn exclude_borders(&self) -> impl Iterator<Item = &[TileCell]> {
        self.data
            .iter()
            .skip(1)
            .rev()
            .skip(1)
            .rev()
            .map(|row| &row[1..TILE_SIZE - 1])
    }
}

impl From<&str> for Tile {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        let id = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .next()
            .unwrap()
            .trim_end_matches(':')
            .parse()
            .unwrap();

        let mut data = [[Default::default(); TILE_SIZE]; TILE_SIZE];
        for (i, data_line) in lines.enumerate() {
            for (j, cell) in data_line.as_bytes().iter().enumerate() {
                if *cell == b'#' {
                    data[i][j] = TileCell(true);
                }
            }
        }

        Self { id, data }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tile {}:", self.id)?;
        for row in &self.data {
            for cell in row {
                if cell.0 {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct Tiles(pub(crate) HashSet<Tile>);

impl From<&str> for Tiles {
    fn from(input: &str) -> Self {
        Self(input.split("\n\n").map(Tile::from).collect())
    }
}

impl Tiles {
    pub(crate) fn corners(&self) -> impl Iterator<Item = (OrdinalDir, &Tile)> + '_ {
        self.0
            .iter()
            .filter_map(move |tile| -> Option<(OrdinalDir, &Tile)> {
                let nbor_dirs: Vec<CardinalDir> = self
                    .0
                    .iter()
                    .filter(|other| tile != *other)
                    .flat_map(|t| Orientations::from(*t))
                    .map(|other| tile.stitch_to(&other))
                    .flatten()
                    .collect();

                OrdinalDir::all()
                    .filter(|od| *od == &nbor_dirs.as_slice())
                    .next()
                    .map(|od| (*od, tile))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_TILE: &'static str = "\
        Tile 2311:\n\
        ..##.#..#.\n\
        ##..#.....\n\
        #...##..#.\n\
        ####.#...#\n\
        ##.##.###.\n\
        ##...#.###\n\
        .#.#.#..##\n\
        ..#....#..\n\
        ###...#.#.\n\
        ..###..###\n";

    #[test]
    fn tile_from_str_works() {
        let tile = Tile::from(EXAMPLE_TILE);
        assert_eq!(tile.id, 2311);

        assert_eq!(tile.data[0], [O, O, X, X, O, X, O, O, X, O]);
        assert_eq!(tile.data[9], [O, O, X, X, X, O, O, X, X, X]);
    }

    #[test]
    fn tile_flip_works() {
        let mut tile = Tile::default();
        tile.data[0][0] = X;
        tile.flip();

        assert_eq!(tile.data[9][0], X);
    }

    #[test]
    fn tile_rotate_works() {
        let mut tile = Tile::default();
        tile.data[0][0] = X;
        tile.rotate();

        assert_eq!(tile.data[0][9], X);
        assert_eq!(tile.data[0][0], O);

        tile.rotate();
        tile.rotate();
        tile.rotate();
        assert_eq!(tile.data[0][9], O);
        assert_eq!(tile.data[0][0], X);
    }

    #[test]
    fn tile_exclude_borders_works() {
        let tile = Tile::from(EXAMPLE_TILE);
        assert_eq!(tile.data[0], [O, O, X, X, O, X, O, O, X, O]);
        assert_eq!(tile.data[TILE_SIZE - 1], [O, O, X, X, X, O, O, X, X, X,]);

        let data: Vec<_> = tile.exclude_borders().collect();
        assert_eq!(data[0].len(), TILE_SIZE - 2);
        assert_eq!(data[0], [X, O, O, X, O, O, O, O]);
        assert_eq!(data[TILE_SIZE - 3], [X, X, O, O, O, X, O, X]);
    }
}
