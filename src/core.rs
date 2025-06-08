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
    NoAutoFinish,
    NoCardsMoved,
    InvalidMove,
    EmptyPile,
    Quit,
    Win,
    Help,
    History,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoAutoFinish => {
                write!(f, "Cannot autofinish, ensure that all cards are uncovered")
            }
            Error::NoCardsMoved => write!(f, "No cards were moved"),
            Error::InvalidMove => write!(f, "Invalid move, try again"),
            Error::EmptyPile => write!(f, "The pile is empty"),
            Error::Quit => write!(f, "Exiting the game"),
            Error::Win => write!(f, "Successfully autofinshed"),
            Error::Help => write!(f, "Help message"),
            Error::History => write!(f, "Move history"),
        }
    }
}
struct HistoryItem {
    table: Table,
    move_played: Move,
}

impl HistoryItem {
    fn new(table: &Table, move_played: Move) -> Self {
        let table = table.clone();
        HistoryItem { table, move_played }
    }
}

pub struct Game {
    table: Table,
    moves: u32,
    history: Vec<HistoryItem>,
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
        self.history.push(HistoryItem::new(&self.table, game_move));
        let move_result = match game_move {
            Move::AutoFinish => self.auto_finish(),
            Move::DrawCard => self.table.draw_card(),
            Move::AutoMove(from, to) => self.table.auto_move(from, to),
            Move::MoveCards(n, from, to) => self.table.move_cards(n, from, to),
            Move::Undo => {
                self.undo();
                Ok(())
            }
            Move::Help => Err(Box::new(Error::Help) as Box<dyn std::error::Error>),
            Move::Quit => Err(Box::new(Error::Quit) as Box<dyn std::error::Error>),
            Move::Invalid => Err(Box::new(Error::InvalidMove) as Box<dyn std::error::Error>),
            Move::History => Err(Box::new(Error::History) as Box<dyn std::error::Error>),
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
                    self.table = t.table;
                    self.moves -= 1
                };
            }
        }
    }

    pub fn print_history(&self) {
        if self.history.is_empty() {
            println!("No moves");
            return;
        }
        let s: String = self
            .history
            .iter()
            .enumerate()
            .map(|(i, item)| format!("{}: {}\n", i, item.move_played))
            .collect();
        println!("{s}");
    }

    fn auto_finish(&mut self) -> Result<()> {
        let lanes_uncovered = (0..7).all(|i| {
            let lane = self.table().lane(i);
            lane.cards().iter().all(|card| !card.is_covered())
        });
        let piles_empty =
            self.table().draw_pile().is_empty() && self.table().uncovered_pile().is_empty();
        if lanes_uncovered && piles_empty {
            let moves: u32 = (0..7).map(|i| self.table().lane(i).length() as u32).sum();
            self.moves += moves;
            Err(Box::new(Error::Win))
        } else {
            Err(Box::new(Error::NoAutoFinish))
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
