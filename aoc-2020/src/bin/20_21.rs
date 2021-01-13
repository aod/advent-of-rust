use solution::{solve_part1, solve_part2};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/21.txt"));

fn main() {
    println!(
        "Part 1: {}\n\
         Part 2: {}",
        solve_part1(INPUT),
        solve_part2(INPUT)
    );
}

mod domain {
    use std::collections::{HashMap, HashSet};

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
    pub struct Ingredient<'a>(pub &'a str);
    pub type Ingredients<'a> = HashSet<Ingredient<'a>>;

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
    pub struct Allergen<'a>(pub &'a str);
    pub type Allergens<'a> = HashSet<Allergen<'a>>;

    #[derive(Default)]
    pub struct Food<'a> {
        pub ingredients: Ingredients<'a>,
        pub allergens: Allergens<'a>,
    }
    pub struct Foods<'a>(pub Vec<Food<'a>>);

    impl<'a> From<&'a str> for Foods<'a> {
        fn from(input: &'a str) -> Self {
            let mut foods: Vec<Food> = Default::default();

            for line in input.lines() {
                let mut split = line.split(" (contains ");
                let ingredients: Ingredients =
                    split.next().unwrap().split(' ').map(Ingredient).collect();
                let allergens: Allergens = split
                    .next()
                    .unwrap()
                    .trim_end_matches(')')
                    .split(", ")
                    .map(Allergen)
                    .collect();

                foods.push(Food {
                    ingredients,
                    allergens,
                })
            }

            Self(foods)
        }
    }

    impl Foods<'_> {
        /// Calculates how many times an ingredient is included in a food with
        /// the specified allergen. Values in the HashMap are sorted in DESC
        /// order by their occurrences.
        fn aler_ingrs_occurrences(&self) -> HashMap<Allergen, Vec<(Ingredient, usize)>> {
            let mut occurs: HashMap<Allergen, HashMap<Ingredient, usize>> = Default::default();
            for food in &self.0 {
                for al in &food.allergens {
                    for igr in &food.ingredients {
                        occurs
                            .entry(*al)
                            .or_insert(Default::default())
                            .entry(*igr)
                            .and_modify(|ocur| *ocur += 1)
                            .or_insert(1);
                    }
                }
            }

            let mut sorted_occurs: HashMap<Allergen, Vec<(Ingredient, usize)>> = Default::default();
            for (al, ingrs) in occurs {
                let mut ingrs: Vec<_> = ingrs.into_iter().collect();
                ingrs.sort_by(|a, b| b.1.cmp(&a.1));

                let (_, highest_occurs) = ingrs[0];
                sorted_occurs.insert(
                    al,
                    ingrs
                        .into_iter()
                        .filter(|x| x.1 == highest_occurs)
                        .collect(),
                );
            }

            sorted_occurs
        }

        pub fn allergenic_ingredients(&self) -> HashMap<Allergen, Ingredient> {
            let mut occurs = self.aler_ingrs_occurrences();
            let mut result: HashMap<Allergen, Ingredient> = Default::default();
            while !occurs.is_empty() {
                let mut res = occurs.clone();
                for (al, ingrs) in &occurs {
                    if match ingrs.as_slice() {
                        [(_, _)] => true,
                        [(_, ocur1), (_, ocur2), ..] if ocur1 > ocur2 => true,
                        _ => false,
                    } {
                        let (ingredient, _) = res.get_mut(al).unwrap().remove(0);
                        res.remove(al);
                        result.insert(*al, ingredient);

                        for (_, ingredients) in res.iter_mut() {
                            if let Some(index) = ingredients
                                .iter()
                                .position(|(other, _)| *other == ingredient)
                            {
                                ingredients.remove(index);
                            }
                        }
                    }
                }
                occurs = res
            }

            result
        }

        fn all_ingredients(&self) -> HashSet<Ingredient> {
            self.0
                .iter()
                .fold(Default::default(), |mut ingredients, food| {
                    ingredients.extend(&food.ingredients);
                    ingredients
                })
        }

        pub fn non_allergenic_ingredients(&self) -> HashSet<Ingredient> {
            let al_ingrs = self
                .allergenic_ingredients()
                .into_iter()
                .map(|x| x.1)
                .collect();

            self.all_ingredients()
                .difference(&al_ingrs)
                .cloned()
                .collect()
        }
    }
}

mod solution {
    use crate::domain::Foods;

    pub fn solve_part1(input: &str) -> usize {
        let foods: Foods = input.into();
        let non_al_ingrs = foods.non_allergenic_ingredients();

        foods.0.iter().fold(0, |count, food| {
            count + non_al_ingrs.intersection(&food.ingredients).count()
        })
    }

    pub fn solve_part2(input: &str) -> String {
        let foods: Foods = input.into();
        let mut al_ingrs: Vec<_> = foods.allergenic_ingredients().into_iter().collect();
        al_ingrs.sort_by(|(a, _), (b, _)| a.cmp(b));

        al_ingrs
            .into_iter()
            .map(|(_, ingr)| ingr.0)
            .collect::<Vec<_>>()
            .join(",")
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::INPUT;

        const EXAMPLE_INPUT: &'static str = "\
            mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\n\
            trh fvjkl sbzzf mxmxvkd (contains dairy)\n\
            sqjhc fvjkl (contains soy)\n\
            sqjhc mxmxvkd sbzzf (contains fish)";

        #[test]
        fn part1_example() {
            assert_eq!(solve_part1(EXAMPLE_INPUT), 5);
        }

        #[test]
        fn part1_answer() {
            assert_eq!(solve_part1(INPUT), 2211);
        }

        #[test]
        fn part2_example() {
            assert_eq!(solve_part2(EXAMPLE_INPUT), "mxmxvkd,sqjhc,fvjkl");
        }

        #[test]
        fn part2_answer() {
            assert_eq!(
                solve_part2(INPUT),
                "vv,nlxsmb,rnbhjk,bvnkk,ttxvphb,qmkz,trmzkcfg,jpvz"
            );
        }
    }
}
