pub mod moves;

use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display};
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
            PileType::Draw => PileRef::clone(&self.draw_pile),
            PileType::Uncovered => PileRef::clone(&self.uncovered_pile),
            PileType::Lane(i) => PileRef::clone(self.lanes.get(i - 1).unwrap()),
            PileType::Suit(suit) => PileRef::clone(self.suit_piles.get(&suit).unwrap()),
        }
    }

    pub fn move_card(&self, card: Card, from: PileRef, to: PileRef) -> Result<(), Box<dyn Error>> {
        let mut from = from.try_borrow_mut()?;
        let mut to = to.try_borrow_mut()?;
        let can_move = to.can_add(&card) && from.can_remove(&card);
        if can_move {
            let from_type = from.pile_type();
            match from_type {
                PileType::Draw => {
                    println!("drawing");
                    to.add_card(card);
                    println!("card added");
                    from.remove_top_card();
                    println!("card removed");
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
        Ok(())
    }

    pub fn move_top_card(&self, from: PileRef, to: PileRef) -> Result<(), Box<dyn Error>> {
        let mut from = from.try_borrow_mut()?;
        let mut to = to.try_borrow_mut()?;
        let mut card = *from.top_card().ok_or(fmt::Error)?;
        let can_move = to.can_add(&card) && from.can_remove(&card);

        if can_move {
            let from_type = from.pile_type();
            match from_type {
                PileType::Draw => {
                    card.flip();
                    to.add_card(card);
                    from.remove_top_card();
                }
                _ => {
                    from.remove_top_card();
                    to.add_card(card);
                    if from.top_card_is_covered() {
                        from.flip_top_card();
                    }
                }
            }
        }
        Ok(())
    }

    pub fn move_cards(
        &self,
        number: usize,
        from: PileRef,
        to: PileRef,
    ) -> Result<(), Box<dyn Error>> {
        if number == 1 {
            return self.move_top_card(Rc::clone(&from), Rc::clone(&to));
        }
        let cards = from.borrow().get_cards(number).clone();
        let results = cards
            .into_iter()
            .map(|card| self.move_card(card, Rc::clone(&from), Rc::clone(&to)))
            .collect::<Vec<Result<(), Box<dyn Error>>>>();
        println!("{:?}", results);
        for res in results {
            if res.is_err() {
                return res;
            } else {
                continue;
            }
        }
        println!("moved {} cards", number);
        Ok(())
    }

    pub fn move_all_cards(&self, from: PileRef, to: PileRef) -> Result<(), Box<dyn Error>> {
        let mut from = from.try_borrow_mut()?;
        let mut to = to.try_borrow_mut()?;
        to.add_all_cards(&mut from.remove_all_cards());
        Ok(())
    }

    pub fn draw_card(&self) -> Result<(), Box<dyn Error>> {
        let mut draw_pile = self.draw_pile.try_borrow_mut()?;
        let mut uncovered_pile = self.uncovered_pile.try_borrow_mut()?;
        match draw_pile.top_card() {
            Some(top_card) => {
                let mut card = *top_card;
                card.flip();
                uncovered_pile.add_card(card);
                draw_pile.remove_top_card();
                Ok(())
            }
            None if uncovered_pile.is_empty() => Ok(()),
            None => {
                draw_pile.add_all_cards(&mut uncovered_pile.remove_all_cards());
                draw_pile.flip_all_cards();
                Ok(())
            }
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

    pub fn auto_move(&self, from: PileRef, to: PileRef) -> Result<(), Box<dyn Error>> {
        let is_valid = (1..to.borrow().length())
            .map(|n| self.is_move_valid(n, Rc::clone(&from), Rc::clone(&to)));
        let index = is_valid
            .into_iter()
            .enumerate()
            .map(|(i, v)| if v { (i + 1) as u8 } else { 0_u8 })
            .filter(|i| *i != 0)
            .collect::<Vec<u8>>();
        let index = index.first().unwrap_or(&0);
        self.move_cards(*index as usize, from, to)
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

    pub fn play(&mut self, game_move: Move) -> Result<(), Box<dyn Error>> {
        match game_move {
            Move::DrawCard => self.table.draw_card(),
            Move::AutoMove(from, to) => self
                .table
                .auto_move(self.table.get_pile(from), self.table.get_pile(to)),
            Move::MoveCards(n, from, to) => {
                self.table
                    .move_cards(n, self.table.get_pile(from), self.table.get_pile(to))
            }
            Move::Invalid => {
                let color_text = ansi_term::Color::Red.paint("Invalid move, try again");
                println!("{}", color_text);
                Ok(())
            }
            m => {
                println!("{:?}", m);
                Ok(())
            }
        }
    }

    pub fn is_over(&self) -> bool {
        self.table()
            .suit_piles
            .values()
            .all(|pile| pile.borrow().length() == 13)
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
