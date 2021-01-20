mod domain;

use aoc_lib::{solve_print, Part1, Part2};
use domain::Tiles;

const INPUT: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/20.txt"));

fn main() {
    solve_print(Box::new(Day20::default()), INPUT);
}

#[derive(Default)]
struct Day20;

impl Part1 for Day20 {
    type A = usize;

    fn solve(&self, input: &str) -> Self::A {
        let tiles = Tiles::from(input);
        tiles
            .0
            .iter()
            .map(|tile| {
                (
                    tiles
                        .0
                        .iter()
                        .filter(|other| *other != tile)
                        .fold(0, |mut nbors, other| {
                            nbors += tile.orientations().fold(0, |mut tile_nbors, tile| {
                                if tile.top_border().eq(other.bottom_border())
                                    || tile.right_border().eq(other.left_border())
                                    || tile.bottom_border().eq(other.top_border())
                                    || tile.left_border().eq(other.right_border())
                                {
                                    tile_nbors += 1
                                }
                                tile_nbors
                            });
                            nbors
                        }),
                    tile,
                )
            })
            .filter(|(nbors, _)| *nbors == 2)
            .map(|(_, tile)| tile.id)
            .product()
    }
}

impl Part2 for Day20 {
    type B = usize;

    fn solve(&self, _input: &str) -> Self::B {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::{Day20, INPUT};
    use aoc_lib::Part1;

    #[test]
    fn part1_example() {
        assert_eq!(
            Part1::solve(&Day20::default(), EXAMPLE_INPUT),
            20899048083289
        );
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(
    //         Part2::solve(&Day21::default(), EXAMPLE_INPUT),
    //         "mxmxvkd,sqjhc,fvjkl"
    //     );
    // }

    #[test]
    fn part1_answer() {
        assert_eq!(Part1::solve(&Day20::default(), INPUT), 19955159604613);
    }

    // #[test]
    // fn part2_answer() {
    //     assert_eq!(
    //         Part2::solve(&Day21::default(), INPUT),
    //         "vv,nlxsmb,rnbhjk,bvnkk,ttxvphb,qmkz,trmzkcfg,jpvz"
    //     );
    // }

    const EXAMPLE_INPUT: &'static str = "\
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
        ..###..###\n\
        \n\
        Tile 1951:\n\
        #.##...##.\n\
        #.####...#\n\
        .....#..##\n\
        #...######\n\
        .##.#....#\n\
        .###.#####\n\
        ###.##.##.\n\
        .###....#.\n\
        ..#.#..#.#\n\
        #...##.#..\n\
        \n\
        Tile 1171:\n\
        ####...##.\n\
        #..##.#..#\n\
        ##.#..#.#.\n\
        .###.####.\n\
        ..###.####\n\
        .##....##.\n\
        .#...####.\n\
        #.##.####.\n\
        ####..#...\n\
        .....##...\n\
        \n\
        Tile 1427:\n\
        ###.##.#..\n\
        .#..#.##..\n\
        .#.##.#..#\n\
        #.#.#.##.#\n\
        ....#...##\n\
        ...##..##.\n\
        ...#.#####\n\
        .#.####.#.\n\
        ..#..###.#\n\
        ..##.#..#.\n\
        \n\
        Tile 1489:\n\
        ##.#.#....\n\
        ..##...#..\n\
        .##..##...\n\
        ..#...#...\n\
        #####...#.\n\
        #..#.#.#.#\n\
        ...#.#.#..\n\
        ##.#...##.\n\
        ..##.##.##\n\
        ###.##.#..\n\
        \n\
        Tile 2473:\n\
        #....####.\n\
        #..#.##...\n\
        #.##..#...\n\
        ######.#.#\n\
        .#...#.#.#\n\
        .#########\n\
        .###.#..#.\n\
        ########.#\n\
        ##...##.#.\n\
        ..###.#.#.\n\
        \n\
        Tile 2971:\n\
        ..#.#....#\n\
        #...###...\n\
        #.#.###...\n\
        ##.##..#..\n\
        .#####..##\n\
        .#..####.#\n\
        #..#.#..#.\n\
        ..####.###\n\
        ..#.#.###.\n\
        ...#.#.#.#\n\
        \n\
        Tile 2729:\n\
        ...#.#.#.#\n\
        ####.#....\n\
        ..#.#.....\n\
        ....#..#.#\n\
        .##..##.#.\n\
        .#.####...\n\
        ####.#.#..\n\
        ##.####...\n\
        ##..#.##..\n\
        #.##...##.\n\
        \n\
        Tile 3079:\n\
        #.#.#####.\n\
        .#..######\n\
        ..#.......\n\
        ######....\n\
        ####.#..#.\n\
        .#...#.##.\n\
        #.#####.##\n\
        ..#.###...\n\
        ..#.......\n\
        ..#.###...\n";
}
