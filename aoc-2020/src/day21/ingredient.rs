use std::collections::HashSet;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub(super) struct Ingredient<'a>(pub &'a str);
pub(super) type Ingredients<'a> = HashSet<Ingredient<'a>>;
