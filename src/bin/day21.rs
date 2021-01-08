use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

type Allergen<'a> = &'a str;
type Ingredient<'a> = &'a str;

type Ingredients<'a> = HashSet<Ingredient<'a>>;
type Foods<'a> = Vec<Ingredients<'a>>;
type FoodAllergens<'a> = HashMap<Allergen<'a>, HashMap<Ingredient<'a>, usize>>;

fn main() {
    let file_path = env::args().skip(1).next().unwrap();
    let input = fs::read_to_string(file_path).unwrap();

    let mut foods: Foods = Default::default();
    let mut foods_allergens: FoodAllergens = Default::default();
    for line in input.lines() {
        let mut split = line.split(" (contains ");
        let raw_ingredients: Ingredients = split.next().unwrap().split(' ').collect();
        foods.push(raw_ingredients.clone());
        let allergens: Vec<Allergen> = split
            .next()
            .unwrap()
            .trim_end_matches(')')
            .split(", ")
            .collect();

        for allergen in allergens {
            for raw_ingredient in &raw_ingredients {
                foods_allergens
                    .entry(allergen)
                    .or_insert(Default::default())
                    .entry(raw_ingredient)
                    .and_modify(|occurrence| *occurrence += 1)
                    .or_insert(1);
            }
        }
    }

    let mut allergenic_ingredients: HashSet<Ingredient> = Default::default();
    let mut non_allergenic_ingredients: HashSet<Ingredient> = Default::default();
    for (_allergen, ingredients) in &foods_allergens {
        let max_occur = ingredients.values().max().unwrap();
        for (ingredient, count) in ingredients {
            if count == max_occur {
                allergenic_ingredients.insert(ingredient);
            } else {
                non_allergenic_ingredients.insert(ingredient);
            }
        }
    }

    let part1_ans: usize = {
        let mut non_allergenic_appearance = 0;
        for ingredients in foods {
            for ingredient in non_allergenic_ingredients.difference(&allergenic_ingredients) {
                if ingredients.contains(ingredient) {
                    non_allergenic_appearance += 1;
                }
            }
        }
        non_allergenic_appearance
    };
    println!("Part 1: {}", part1_ans);

    let part2_ans = {
        let mut allergen_ingredient: HashMap<Allergen, Ingredient> = Default::default();

        while !allergenic_ingredients.is_empty() {
            for (allergen, ingredients) in &foods_allergens {
                if allergen_ingredient.contains_key(allergen) {
                    continue;
                }

                let mut ingredients: Vec<_> = ingredients
                    .iter()
                    .filter(|(ingredient, _)| {
                        !allergen_ingredient
                            .values()
                            .any(|other| *ingredient == other)
                    })
                    .collect();
                ingredients.sort_by(|a, b| b.1.cmp(a.1));

                match ingredients.as_slice() {
                    [(ingredient, _)] => {
                        if allergenic_ingredients.remove(*ingredient) {
                            allergen_ingredient.insert(allergen, ingredient);
                        }
                    }
                    [(ingr1, ocur1), (_, ocur2), ..] if ocur1 > ocur2 => {
                        if allergenic_ingredients.remove(*ingr1) {
                            allergen_ingredient.insert(allergen, ingr1);
                        }
                    }
                    _ => {}
                };
            }
        }

        let mut allergen_ingredient: Vec<_> = allergen_ingredient.iter().collect();
        allergen_ingredient.sort_by(|a, b| a.0.cmp(b.0));

        allergen_ingredient
            .iter()
            .map(|(_, ingredient)| **ingredient)
            .collect::<Vec<_>>()
            .join(",")
    };
    println!("Part 2: {}", part2_ans);
}
