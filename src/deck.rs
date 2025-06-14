pub mod card;

use std::fmt::{Display, Formatter, Result};

use card::{Card, Rank, Suit};
use rand::{rng, seq::SliceRandom};
use strum::IntoEnumIterator;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Suit::iter()
            .flat_map(|suit| Rank::iter().map(move |rank| Card::new(rank, suit)))
            .collect::<Vec<Card>>();

        cards.shuffle(&mut rng());
        Deck { cards }
    }

    pub fn pick_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn pick_cards(&mut self, number: u8) -> Vec<Card> {
        (0..number)
            .filter_map(move |_| self.pick_card())
            .collect()
    }

    pub fn pick_all_cards(self) -> Vec<Card> {
        self.cards
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.cards.is_empty() {
            return write!(f, "   ");
        }
        let s: String = self
            .cards
            .iter()
            .map(|card| format!("{}\n", card))
            .collect();
        write!(f, "{}", s)
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}
