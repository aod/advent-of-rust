pub(crate) const TILE_SIZE: usize = 10;

/// Tile - A small image from camera array
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Tile {
    /// Unieuqe identifier of the tile
    pub(crate) id: usize,
    /// The image chunk data where true = `#` and false = `.`
    pub(crate) data: [[bool; TILE_SIZE]; TILE_SIZE],
}

impl Tile {
    pub(crate) fn flip(&mut self) {
        self.data.reverse()
    }

    pub(crate) fn rotate(&mut self) {
        let mut rotated: [[bool; TILE_SIZE]; TILE_SIZE] = Default::default();
        for i in 0..TILE_SIZE {
            for j in 0..TILE_SIZE {
                rotated[i][j] = self.data[TILE_SIZE - j - 1][i];
            }
        }

        self.data = rotated;
    }

    pub(crate) fn top_border(&self) -> impl Iterator<Item = &bool> {
        self.data[0].iter()
    }

    pub(crate) fn bottom_border(&self) -> impl Iterator<Item = &bool> {
        self.data[TILE_SIZE - 1].iter()
    }

    pub(crate) fn right_border(&self) -> impl Iterator<Item = &bool> {
        self.data.iter().map(|row| &row[TILE_SIZE - 1])
    }

    pub(crate) fn left_border(&self) -> impl Iterator<Item = &bool> {
        self.data.iter().map(|row| &row[0])
    }

    pub(crate) fn orientations(self) -> impl Iterator<Item = Tile> {
        TileOrientations::from(self)
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

        let mut data = [[false; TILE_SIZE]; TILE_SIZE];
        for (i, data_line) in lines.enumerate() {
            for (j, cell) in data_line.as_bytes().iter().enumerate() {
                if *cell == b'#' {
                    data[i][j] = true;
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
                if *cell {
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
pub(crate) struct Tiles(pub(crate) Vec<Tile>);

impl From<&str> for Tiles {
    fn from(input: &str) -> Self {
        Self(input.split("\n\n").map(Tile::from).collect())
    }
}

#[derive(Debug)]
pub(crate) struct TileOrientations {
    tile: Tile,
    rotation_count: usize,
}

impl From<Tile> for TileOrientations {
    fn from(tile: Tile) -> Self {
        Self {
            tile,
            rotation_count: 0,
        }
    }
}

impl Iterator for TileOrientations {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rotation_count >= 8 {
            return None;
        }

        if self.rotation_count % 2 == 0 {
            self.tile.flip();
        } else {
            self.tile.flip();
            self.tile.rotate();
        }

        self.rotation_count += 1;
        Some(self.tile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_from_str_works() {
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

        let tile = Tile::from(EXAMPLE_TILE);
        assert_eq!(tile.id, 2311);

        const O: bool = false;
        const X: bool = true;
        assert_eq!(tile.data[0], [O, O, X, X, O, X, O, O, X, O]);
        assert_eq!(tile.data[9], [O, O, X, X, X, O, O, X, X, X]);
    }

    #[test]
    fn tile_flip_works() {
        let mut tile = Tile::default();
        tile.data[0][0] = true;
        tile.flip();

        assert_eq!(tile.data[9][0], true);
    }

    #[test]
    fn tile_rotate_works() {
        let mut tile = Tile::default();
        tile.data[0][0] = true;
        tile.rotate();

        assert_eq!(tile.data[0][9], true);
        assert_eq!(tile.data[0][0], false);

        tile.rotate();
        tile.rotate();
        tile.rotate();
        assert_eq!(tile.data[0][9], false);
        assert_eq!(tile.data[0][0], true);
    }
}
