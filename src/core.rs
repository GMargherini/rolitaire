pub mod moves;

use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

use strum::IntoEnumIterator;

use crate::core::moves::Move;
use crate::deck::card::{Card, Suit};
use crate::deck::{Deck, card};
use crate::pile::{Pile, PileType};

type PileRef = Rc<RefCell<Pile>>;

pub struct Table {
    suit_piles: HashMap<card::Suit, PileRef>,
    lanes: Vec<PileRef>,
    draw_pile: PileRef,
    uncovered_pile: PileRef,
}

impl Table {
    fn new() -> Table {
        let mut deck = Deck::new();
        let mut suit_piles = HashMap::new();
        Suit::iter()
            .map(|suit| {
                (
                    suit,
                    Rc::new(RefCell::new(Pile::new(vec![], PileType::Suit(suit)))),
                )
            })
            .map(|pair| suit_piles.insert(pair.0, pair.1))
            .count();
        let mut lanes: Vec<PileRef> = (1..=7)
            .map(|i| {
                Rc::new(RefCell::new(Pile::new(
                    deck.pick_cards(i),
                    PileType::Lane(i as usize),
                )))
            })
            .collect();
        lanes
            .iter_mut()
            .for_each(|lane| lane.borrow_mut().flip_top_card());
        let uncovered_pile = Rc::new(RefCell::new(Pile::new(vec![], PileType::Uncovered)));
        let draw_pile = Rc::new(RefCell::new(Pile::new(
            deck.pick_all_cards(),
            PileType::Draw,
        )));

        Table {
            suit_piles,
            lanes,
            draw_pile,
            uncovered_pile,
        }
    }

    pub fn draw_pile(&self) -> Ref<'_, Pile> {
        self.draw_pile.borrow()
    }

    pub fn uncovered_pile(&self) -> Ref<'_, Pile> {
        self.uncovered_pile.borrow()
    }

    pub fn lane(&self, index: usize) -> Ref<'_, Pile> {
        self.lanes[index].borrow()
    }

    pub fn suit_pile(&self, suit: Suit) -> Ref<'_, Pile> {
        self.suit_piles[&suit].borrow()
    }

    fn get_pile(&self, pile_type: PileType) -> PileRef {
        match pile_type {
            PileType::Draw => self.draw_pile.clone(),
            PileType::Uncovered => self.uncovered_pile.clone(),
            PileType::Lane(i) => self.lanes.get((i - 1) as usize).unwrap().clone(),
            PileType::Suit(suit) => self.suit_piles.get(&suit).unwrap().clone(),
        }
    }

    pub fn move_card(&self, card: Card, from: PileRef, to: PileRef) {
        let mut to = to.borrow_mut();
        let mut from = from.borrow_mut();
        if to.can_add(&card) && from.can_remove(&card) {
            match from.pile_type() {
                PileType::Draw => {
                    to.add_card(card);
                    from.remove_top_card();
                }
                _ => {
                    from.remove_card(&card);
                    to.add_card(card);
                    if from.top_card_is_covered() {
                        from.flip_top_card();
                    }
                }
            }
        }
    }

    pub fn move_cards(&self, number: usize, from: PileRef, to: PileRef) {
        // let mut from = from.borrow_mut();
        from.borrow_mut()
            .remove_cards(number)
            .into_iter()
            .for_each(|card| self.move_card(card, from.clone(), to.clone()));
    }

    pub fn move_all_cards(&self, from: PileRef, to: PileRef) {
        let mut from = from.borrow_mut();
        let mut to = to.borrow_mut();
        to.add_all_cards(&mut from.remove_all_cards());
    }

    pub fn draw_card(&self) {
        match (
            self.draw_pile.borrow().pile_type(),
            self.uncovered_pile.borrow().pile_type(),
        ) {
            (PileType::Draw, PileType::Uncovered) => match self.draw_pile.borrow().top_card() {
                Some(top_card) => self.move_card(
                    top_card.clone(),
                    self.draw_pile.clone(),
                    self.uncovered_pile.clone(),
                ),
                None if self.uncovered_pile.borrow().is_empty() => {}
                None => {
                    self.move_all_cards(self.uncovered_pile.clone(), self.draw_pile.clone());
                    self.draw_pile.borrow_mut().flip_all_cards();
                }
            },
            _ => (),
        }
    }

    fn is_move_valid(&self, number: usize, from: PileRef, to: PileRef) -> bool {
        if number == 0 {
            return false;
        }
        match from.borrow().card(from.borrow().length() - number) {
            Some(card) => to.borrow().can_add(card),
            None => true,
        }
    }

    pub fn auto_move(&self, from: PileRef, to: PileRef) {
        let is_valid =
            (1..to.borrow().length()).map(|n| self.is_move_valid(n, from.clone(), to.clone()));
        let index = is_valid
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                if v {
                    return (i + 1) as u8;
                } else {
                    return 0 as u8;
                }
            })
            .filter(|i| *i != 0)
            .collect::<Vec<u8>>();
        let index = index.first().unwrap_or(&0);
        self.move_cards(*index as usize, from, to);
    }
}

pub struct Game {
    table: Table,
}

impl Game {
    pub fn new() -> Game {
        let table = Table::new();
        Game { table }
    }

    pub fn table(&self) -> &Table {
        &self.table
    }

    pub fn play(&mut self, game_move: Move) {
        match game_move {
            Move::DrawCard => self.table.draw_card(),
            Move::AutoMove(from, to) => self
                .table
                .auto_move(self.table.get_pile(from), self.table.get_pile(to)),
            Move::MoveCards(n, from, to) => {
                self.table
                    .move_cards(n, self.table.get_pile(from), self.table.get_pile(to))
            }
            m => {
                println!("{:?}", m);
                todo!()
            }
        }
    }

    pub fn is_over(&self) -> bool {
        self.table()
            .suit_piles
            .iter()
            .map(|(_, pile)| pile.borrow().length() == 13)
            .fold(true, |x, y| x && y)
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
