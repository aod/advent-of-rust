mod domain;

use aoc_lib::{solve_print, Part1, Part2};
use domain::Foods;

const INPUT: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/21.txt"));

fn main() {
    solve_print(Box::new(Day21::default()), INPUT);
}

#[derive(Default)]
pub(crate) struct Day21;

impl Part1 for Day21 {
    type A = usize;

    fn solve(&self, input: &str) -> Self::A {
        let foods = Foods::from(input);
        let non_al_ingrs = foods.non_allergenic_ingredients();

        foods.0.iter().fold(0, |count, food| {
            count + non_al_ingrs.intersection(&food.ingredients).count()
        })
    }
}

impl Part2 for Day21 {
    type B = String;

    fn solve(&self, input: &str) -> Self::B {
        let foods = Foods::from(input);
        let mut al_ingrs: Vec<_> = foods.allergenic_ingredients().into_iter().collect();
        al_ingrs.sort_by(|(a, _), (b, _)| a.cmp(b));

        al_ingrs
            .into_iter()
            .map(|(_, ingr)| ingr.0)
            .collect::<Vec<_>>()
            .join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::{Day21, INPUT};
    use aoc_lib::{Part1, Part2};

    const EXAMPLE_INPUT: &'static str = "\
            mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\n\
            trh fvjkl sbzzf mxmxvkd (contains dairy)\n\
            sqjhc fvjkl (contains soy)\n\
            sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn part1_example() {
        assert_eq!(Part1::solve(&Day21::default(), EXAMPLE_INPUT), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            Part2::solve(&Day21::default(), EXAMPLE_INPUT),
            "mxmxvkd,sqjhc,fvjkl"
        );
    }

    #[test]
    fn part1_answer() {
        assert_eq!(Part1::solve(&Day21::default(), INPUT), 2211);
    }

    #[test]
    fn part2_answer() {
        assert_eq!(
            Part2::solve(&Day21::default(), INPUT),
            "vv,nlxsmb,rnbhjk,bvnkk,ttxvphb,qmkz,trmzkcfg,jpvz"
        );
    }
}
