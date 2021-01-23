use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub(super) struct Ingredient<'a>(pub &'a str);
pub(super) type Ingredients<'a> = HashSet<Ingredient<'a>>;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub(super) struct Allergen<'a>(pub &'a str);
pub(super) type Allergens<'a> = HashSet<Allergen<'a>>;

#[derive(Default)]
pub(super) struct Food<'a> {
    pub(super) ingredients: Ingredients<'a>,
    pub(super) allergens: Allergens<'a>,
}
pub(super) struct Foods<'a>(pub(super) Vec<Food<'a>>);

impl<'a> From<&'a str> for Foods<'a> {
    fn from(input: &'a str) -> Self {
        let mut foods: Vec<Food> = Default::default();

        for line in input.trim().lines() {
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
                        .or_insert_with(Default::default)
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

    pub(super) fn allergenic_ingredients(&self) -> HashMap<Allergen, Ingredient> {
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

    pub(super) fn non_allergenic_ingredients(&self) -> HashSet<Ingredient> {
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
