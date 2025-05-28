use std::{
    fmt::{Display, Error, Formatter},
    u8,
};
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, Clone, Copy, EnumIter, PartialEq)]
pub enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

#[derive(Debug, Clone, Copy)]
pub enum Colour {
    Black,
    Red,
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}
impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card { rank, suit }
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn colour(&self) -> Colour {
        match self.suit {
            Suit::Clubs | Suit::Spades => Colour::Black,
            Suit::Diamonds | Suit::Hearts => Colour::Red,
        }
    }
}
impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let suit = match self {
            Suit::Clubs => "C",
            Suit::Diamonds => "D",
            Suit::Hearts => "H",
            Suit::Spades => "S",
        };

        write!(f, "{}", suit)
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let rank = match self {
            Rank::Ace => " A",
            Rank::Jack => " J",
            Rank::Queen => " Q",
            Rank::King => " K",
            r if r != &Rank::Ten => &format!(" {}", *r as u8),
            r => &format!("{}", *r as u8),
        };
        write!(f, "{}", rank)
    }
}
impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let card = format!("{}{}", self.rank, self.suit);

        let ansi = match self.colour() {
            Colour::Red => ansi_term::Colour::Red.paint(card),
            Colour::Black => ansi_term::Colour::White.paint(card),
        };
        write!(f, "{}", ansi)
    }
}
