use std::collections::HashMap;

use super::{
    direction::CardinalDir,
    orient::{Orientable, Orientations},
    tile::{Tile, TileCell, Tiles, O, TILE_SIZE, X},
};

pub(crate) const SEA_MONSTER_X_COUNT: usize = 15;
pub(crate) const SEA_MONSTER_HEIGHT: usize = 3;
pub(crate) const SEA_MONSTER_WIDTH: usize = 20;
pub(crate) const SEA_MONSTER: [[TileCell; SEA_MONSTER_WIDTH]; SEA_MONSTER_HEIGHT] = {
    [
        // The monster:
        // |                  # |
        // |#    ##    ##    ###|
        // | #  #  #  #  #  #   |
        [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, X, O],
        [X, O, O, O, O, X, X, O, O, O, O, X, X, O, O, O, O, X, X, X],
        [O, X, O, O, X, O, O, X, O, O, X, O, O, X, O, O, X, O, O, O],
    ]
};

#[derive(Debug, Default, Clone)]
pub(crate) struct Image(pub(crate) Vec<Vec<TileCell>>);

impl Orientable for Image {
    fn flip(&mut self) {
        self.0.reverse();
    }

    fn rotate(&mut self) {
        let mut rotated: Vec<Vec<TileCell>> = Default::default();
        let n = self.height();
        for i in 0..n {
            rotated.push(vec![Default::default(); n]);
            for j in 0..n {
                rotated[i][j] = self.0[n - j - 1][i];
            }
        }

        self.0 = rotated;
    }
}

impl Image {
    fn line_up_tiles(mut tiles: Tiles) -> Vec<Vec<Tile>> {
        let mut image_pos_tiles: HashMap<(isize, isize), Tile> = Default::default();
        let mut q: Vec<((isize, isize), Tile)> = Default::default();

        {
            let start = *tiles.0.iter().next().unwrap();
            tiles.0.remove(&start);
            image_pos_tiles.insert((0, 0), start);
            q.push(((0, 0), start));
        }

        let mut hor_pos = (0, 0); // low, high
        let mut ver_pos = (0, 0); // low, high
        while let Some((coord, tile)) = q.pop() {
            let mut found = vec![];
            for other in &tiles.0 {
                for orient in Orientations::from(*other) {
                    if let Some(dir) = tile.stitch_to(&orient) {
                        let d = match dir {
                            CardinalDir::North => (0, 1),
                            CardinalDir::East => (1, 0),
                            CardinalDir::South => (0, -1),
                            CardinalDir::West => (-1, 0),
                        };
                        let orient_pos = (coord.0 + d.0, coord.1 + d.1);

                        hor_pos.0 = hor_pos.0.min(orient_pos.0);
                        hor_pos.1 = hor_pos.1.max(orient_pos.0);

                        ver_pos.0 = ver_pos.0.min(orient_pos.1);
                        ver_pos.1 = ver_pos.1.max(orient_pos.1);

                        image_pos_tiles.insert(orient_pos, orient);
                        q.push((orient_pos, orient));
                        found.push(*other);
                        break;
                    }
                }
            }

            for f in found {
                tiles.0.remove(&f);
            }
        }

        (ver_pos.0..=ver_pos.1)
            .map(|y| {
                (hor_pos.0..=hor_pos.1)
                    .map(|x| *image_pos_tiles.get(&(x, y)).unwrap())
                    .collect()
            })
            .collect()
    }

    pub(crate) fn width(&self) -> usize {
        self.0[0].len()
    }

    pub(crate) fn height(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn sea_monsters(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height() - SEA_MONSTER_HEIGHT {
            'monster_check: for x in 0..self.width() - SEA_MONSTER_WIDTH {
                for sy in 0..SEA_MONSTER_HEIGHT {
                    let monster_row = SEA_MONSTER[sy]
                        .iter()
                        .enumerate()
                        .filter(|(_, tile_cell)| **tile_cell == X);
                    let tile_row = self.0[y + sy][x..]
                        .iter()
                        .take(SEA_MONSTER_WIDTH)
                        .enumerate()
                        .filter(|(_, tile_cell)| **tile_cell == X);

                    for x in monster_row {
                        if tile_row.clone().find(|v| *v == x).is_none() {
                            continue 'monster_check;
                        }
                    }
                }
                count += 1;
            }
        }

        count
    }
}

impl From<Tiles> for Image {
    fn from(tiles: Tiles) -> Self {
        let image_tiles = Image::line_up_tiles(tiles);

        let mut image: Vec<Vec<TileCell>> = Default::default();
        for row in image_tiles.iter().rev() {
            let mut datas = row
                .iter()
                .map(|tile| tile.exclude_borders())
                .collect::<Vec<_>>();

            for _ in 0..TILE_SIZE - 2 {
                let mut image_row = vec![];
                for data in datas.iter_mut() {
                    image_row.extend(data.next().unwrap().iter());
                }
                image.push(image_row);
            }
        }

        Self(image)
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for tile_cell in row {
                write!(f, "{}", tile_cell)?
            }
            writeln!(f)?
        }

        Ok(())
    }
}
