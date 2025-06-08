use std::fmt::{Display, Error, Formatter};

use crate::deck::card::{self, Card, Rank};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PileType {
    Lane(usize),
    Suit(card::Suit),
    Draw,
    Uncovered,
}

impl Display for PileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pile = match self {
            PileType::Lane(n) => &format!("{n}")[..],
            PileType::Suit(s) => match s {
                card::Suit::Clubs => "C",
                card::Suit::Diamonds => "D",
                card::Suit::Hearts => "H",
                card::Suit::Spades => "S",
            },
            PileType::Draw => "D",
            PileType::Uncovered => "P",
        };
        write!(f, "{}", pile)
    }
}
#[derive(Debug, Clone)]
pub struct Pile {
    cards: Vec<Card>,
    pile_type: PileType,
}

impl Pile {
    pub fn new(cards: Vec<Card>, pile_type: PileType) -> Pile {
        Pile { cards, pile_type }
    }

    pub fn flip_top_card(&mut self) -> Pile {
        if let Some(card) = self.cards.last_mut() {
            card.flip();
        }
        Pile {
            cards: self.cards(),
            pile_type: self.pile_type,
        }
    }

    pub fn flip_all_cards(&mut self) {
        self.cards.iter_mut().for_each(|card| card.flip());
    }

    pub fn length(&self) -> usize {
        self.cards.len()
    }

    pub fn card(&self, index: usize) -> Option<&Card> {
        self.cards.get(index)
    }

    pub fn get_cards(&self, number: usize) -> Vec<Card> {
        let removed = self.cards.split_at(self.cards.len() - number).1;
        Vec::from(removed)
    }

    pub fn cards(&self) -> Vec<Card> {
        self.cards.clone()
    }

    pub fn pile_type(&self) -> PileType {
        self.pile_type
    }

    pub fn top_card(&self) -> Option<&Card> {
        self.cards.last()
    }
    pub fn can_add(&self, card: &Card) -> bool {
        match self.pile_type {
            PileType::Uncovered => true,
            PileType::Draw => true,
            PileType::Suit(suit) => match self.top_card() {
                Some(top_card) => card.suit() == suit && card.is_next(top_card),
                None => card.suit() == suit && card.rank() == Rank::Ace,
            },
            PileType::Lane(_) => match self.top_card() {
                Some(top_card) => top_card.colour() != card.colour() && top_card.is_next(card),
                None => card.rank() == Rank::King,
            },
        }
    }

    pub fn can_remove(&self, card: &Card) -> bool {
        match self.pile_type {
            PileType::Draw => self.cards.contains(card) && card.is_covered(),
            _ => self.cards.contains(card) && !card.is_covered(),
        }
    }

    pub fn remove_card(&mut self, card: &Card) -> Option<Card> {
        if let Some(index) = self.cards.iter().position(|c| c == card) {
            Some(self.cards.remove(index))
        } else {
            None
        }
    }

    pub fn remove_all_cards(&mut self) -> Vec<Card> {
        let cards = self.cards.clone();
        self.cards.clear();
        cards
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn add_all_cards(&mut self, cards: &mut Vec<Card>) {
        self.cards.append(cards);
    }

    pub fn remove_top_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn top_card_is_covered(&self) -> bool {
        match self.cards.last() {
            Some(card) => card.is_covered(),
            None => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn reverse(&mut self) {
        self.cards.reverse();
    }
}

impl Display for Pile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let s = match self.pile_type {
            PileType::Draw | PileType::Uncovered | PileType::Suit(_) => match self.cards.last() {
                Some(card) => card.to_string(),
                None => "░░░".to_string(),
            },
            PileType::Lane(i) => format!("{}", i),
        };
        write!(f, "{}", s)
    }
}
