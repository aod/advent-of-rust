mod domain;

use aoc_lib::{Part1, Part2, Solution};
use domain::{BlackTiles, Tiles};

const INPUT: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/24.txt"));

fn main() {
    Day24::default().solve_print(INPUT);
}

#[derive(Default)]
struct Day24;

impl Part1 for Day24 {
    type A = usize;

    fn solve(&self, input: &str) -> Self::A {
        BlackTiles::flip_tiles(&Tiles::from(input)).0.len()
    }
}

impl Part2 for Day24 {
    type B = usize;

    fn solve(&self, input: &str) -> Self::B {
        BlackTiles::flip_tiles(&Tiles::from(input))
            .nth(99)
            .unwrap()
            .0
            .len()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Day24, INPUT};
    use aoc_lib::{Part1, Part2};

    #[test]
    fn part1_example() {
        assert_eq!(Part1::solve(&Day24::default(), EXAMPLE_INPUT), 10);
    }

    #[test]
    fn part2_example() {
        assert_eq!(Part2::solve(&Day24::default(), EXAMPLE_INPUT), 2208);
    }

    #[test]
    fn part1_answer() {
        assert_eq!(Part1::solve(&Day24::default(), INPUT), 469);
    }

    #[test]
    fn part2_answer() {
        assert_eq!(Part2::solve(&Day24::default(), INPUT), 4353);
    }

    const EXAMPLE_INPUT: &str = "\
        sesenwnenenewseeswwswswwnenewsewsw\n\
        neeenesenwnwwswnenewnwwsewnenwseswesw\n\
        seswneswswsenwwnwse\n\
        nwnwneseeswswnenewneswwnewseswneseene\n\
        swweswneswnenwsewnwneneseenw\n\
        eesenwseswswnenwswnwnwsewwnwsene\n\
        sewnenenenesenwsewnenwwwse\n\
        wenwwweseeeweswwwnwwe\n\
        wsweesenenewnwwnwsenewsenwwsesesenwne\n\
        neeswseenwwswnwswswnw\n\
        nenwswwsewswnenenewsenwsenwnesesenew\n\
        enewnwewneswsewnwswenweswnenwsenwsw\n\
        sweneswneswneneenwnewenewwneswswnese\n\
        swwesenesewenwneswnwwneseswwne\n\
        enesenwswwswneneswsenwnewswseenwsese\n\
        wnwnesenesenenwwnenwsewesewsesesew\n\
        nenewswnwewswnenesenwnesewesw\n\
        eneswnwswnwsenenwnwnwwseeswneewsenese\n\
        neswnwewnwnwseenwseesewsenwsweewe\n\
        wseweeenwnesenwwwswnew";
}
