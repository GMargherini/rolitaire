use std::fmt::{Display, Error, Formatter};
use strum_macros::{EnumIter, FromRepr};

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, FromRepr)]
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

impl Rank {
    pub fn is_next(r1: &Rank, r2: &Rank) -> bool {
        match r2 {
            Rank::King => false,
            _ => &Rank::from_repr((*r2 as usize) + 1).unwrap() == r1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Covered,
    Uncovered,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colour {
    Black,
    Red,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Card {
    rank: Rank,
    suit: Suit,
    state: State,
}
impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card {
            rank,
            suit,
            state: State::Covered,
        }
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

    pub fn is_covered(&self) -> bool {
        match self.state {
            State::Covered => true,
            State::Uncovered => false,
        }
    }

    pub fn flip(&mut self) {
        self.state = match self.state {
            State::Covered => State::Uncovered,
            State::Uncovered => State::Covered,
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
        if self.is_covered() {
            return write!(f, "\u{2587}\u{2587}\u{2587}");
        }
        let card = format!("{}{}", self.rank, self.suit);

        let ansi = match self.colour() {
            Colour::Red => ansi_term::Colour::Red.paint(card),
            Colour::Black => ansi_term::Colour::White.paint(card),
        };
        write!(f, "{}", ansi)
    }
}
