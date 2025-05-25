pub mod card;

use std::fmt::Display;

use card::{Card, Rank, Suit};
use rand::{rng, seq::SliceRandom};
use strum::IntoEnumIterator;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = Suit::iter()
            .flat_map(|suit| Rank::iter().map(move |rank| Card::new(rank, suit)))
            .collect();
        cards.shuffle(&mut rng());
        Deck { cards }
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self
            .cards
            .iter()
            .map(|card| format!("{}\n", card.to_string()))
            .collect();
        write!(f, "{}", s)
    }
}
