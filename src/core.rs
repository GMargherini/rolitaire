pub mod moves;
pub mod table;

use std::{
    cell::RefCell,
    fmt::{self, Display},
    rc::Rc,
};

use strum::IntoEnumIterator;

use crate::{
    core::{moves::Move, table::Table},
    deck::card::Suit,
    pile::Pile,
};

type PileRef = Rc<RefCell<Pile>>;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum Error {
    NoCardsMoved,
    InvalidMove,
    EmptyPile,
    Quit,
    Help,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoCardsMoved => write!(f, "No cards were moved"),
            Error::InvalidMove => write!(f, "Invalid move, try again"),
            Error::EmptyPile => write!(f, "The pile is empty"),
            Error::Quit => write!(f, "Exiting the game"),
            Error::Help => write!(f, "Help message"),
        }
    }
}

pub struct Game {
    table: Table,
    moves: u32,
    history: Vec<Table>,
}

impl Game {
    pub fn new() -> Game {
        let table = Table::new();
        Game {
            table,
            moves: 0,
            history: vec![],
        }
    }

    pub fn table(&self) -> &Table {
        &self.table
    }

    pub fn play(&mut self, game_move: Move) -> Result<()> {
        self.history.push(self.table.clone());
        let move_result = match game_move {
            Move::DrawCard => self.table.draw_card(),
            Move::AutoMove(from, to) => self
                .table
                .auto_move(self.table.get_pile(from), self.table.get_pile(to)),
            Move::MoveCards(n, from, to) => {
                self.table
                    .move_cards(n, self.table.get_pile(from), self.table.get_pile(to))
            }
            Move::Undo => {
                self.undo();
                Ok(())
            }
            Move::Help => Err(Box::new(Error::Help) as Box<dyn std::error::Error>),
            Move::Quit => Err(Box::new(Error::Quit) as Box<dyn std::error::Error>),
            Move::Invalid => Err(Box::new(Error::InvalidMove) as Box<dyn std::error::Error>),
            Move::History => todo!(),
        };
        if move_result.is_ok() {
            match game_move {
                Move::Undo | Move::History => (),
                _ => self.moves += 1,
            }
        } else {
            self.history.pop();
        }
        move_result
    }

    fn undo(&mut self) {
        match self.moves {
            0 => {
                self.moves = 0;
            }
            _ => {
                self.history.pop();
                if let Some(t) = self.history.pop() {
                    self.table = t;
                    self.moves -= 1
                };
            }
        }
    }

    pub fn is_over(&self) -> bool {
        let mut suit_piles = Suit::iter().map(|suit| self.table().suit_pile(suit));
        suit_piles.all(|pile| pile.length() == 13)
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Moves: {}", self.moves)
    }
}
