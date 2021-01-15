use std::collections::{HashSet, VecDeque};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(super) struct Card(pub usize);

#[derive(Default, Clone, PartialEq, Eq, Hash, Debug)]
pub(super) struct Deck(pub VecDeque<Card>);

impl Deck {
    pub(super) fn score(&self) -> usize {
        self.0.iter().rev().zip(1..).map(|(c, m)| m * c.0).sum()
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(super) struct Game(Deck, Deck);

impl From<&str> for Game {
    fn from(input: &str) -> Self {
        let mut decks: Vec<_> = input
            .trim()
            .split("\n\n")
            .map(|chunk| {
                chunk
                    .split("\n")
                    .skip(1)
                    .map(|line| line.parse().unwrap())
                    .map(Card)
                    .collect()
            })
            .collect();

        Self(Deck(decks.remove(0)), Deck(decks.remove(0)))
    }
}

impl Game {
    pub(super) fn simulate_combat(&self) -> Deck {
        let mut game_copy = (*self).clone();
        while !game_copy.0 .0.is_empty() && !game_copy.1 .0.is_empty() {
            let c1 = game_copy.0 .0.pop_front().unwrap();
            let c2 = game_copy.1 .0.pop_front().unwrap();
            if c1 > c2 {
                game_copy.0 .0.extend([c1, c2].iter());
            } else {
                game_copy.1 .0.extend([c2, c1].iter());
            }
        }

        if game_copy.0 .0.is_empty() {
            game_copy.1
        } else {
            game_copy.0
        }
    }
}

impl Game {
    /// Returns true if first deck wins the game.
    fn recursive_combat(game: &mut Game) -> bool {
        let mut seen_games: HashSet<Game> = Default::default();

        loop {
            let ref mut deck1 = game.0;
            let ref mut deck2 = game.1;

            let card1 = deck1.0.pop_front().unwrap();
            let card2 = deck2.0.pop_front().unwrap();

            let won = {
                if card1.0 <= deck1.0.len() && card2.0 <= deck2.0.len() {
                    Self::recursive_combat(&mut Game(
                        Deck(deck1.0.iter().copied().take(card1.0).collect()),
                        Deck(deck2.0.iter().copied().take(card2.0).collect()),
                    ))
                } else {
                    card1 > card2
                }
            };

            if won {
                deck1.0.extend([card1, card2].iter());
            } else {
                deck2.0.extend([card2, card1].iter());
            }

            if !seen_games.insert(game.clone()) {
                return true;
            }

            if game.0 .0.is_empty() || game.1 .0.is_empty() {
                break;
            }
        }

        !game.0 .0.is_empty()
    }

    pub(super) fn simulate_recursive_combat(&self) -> Deck {
        let mut game_copy = (*self).clone();
        if Self::recursive_combat(&mut game_copy) {
            game_copy.0
        } else {
            game_copy.1
        }
    }
}
