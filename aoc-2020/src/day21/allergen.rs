use std::collections::HashSet;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub(super) struct Allergen<'a>(pub &'a str);
pub(super) type Allergens<'a> = HashSet<Allergen<'a>>;
