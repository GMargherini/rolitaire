use super::{Error, PileRef, Result};
use crate::{
    deck::{
        Deck,
        card::{self, Card, Suit},
    },
    pile::{Pile, PileType},
};
use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};
use strum::IntoEnumIterator;
#[derive(Debug)]
pub struct Table {
    suit_piles: HashMap<card::Suit, PileRef>,
    lanes: Vec<PileRef>,
    draw_pile: PileRef,
    uncovered_pile: PileRef,
}

impl Table {
    pub fn new() -> Table {
        let mut deck = Deck::new();
        let mut suit_piles = HashMap::new();
        for suit in Suit::iter() {
            let pile = Rc::new(RefCell::new(Pile::new(vec![], PileType::Suit(suit))));
            suit_piles.insert(suit, pile);
        }
        let lanes: Vec<PileRef> = (1..=7)
            .map(|i| Pile::new(deck.pick_cards(i), PileType::Lane(i as usize)))
            .map(|mut lane| lane.flip_top_card())
            .map(|lane| Rc::new(RefCell::new(lane)))
            .collect();
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

    fn move_card(&self, card: Card, from: PileRef, to: PileRef) -> Result<()> {
        let mut from = from.try_borrow_mut()?;
        let mut to = to.try_borrow_mut()?;
        if let PileType::Uncovered = to.pile_type() {
            return Err(Box::new(Error::InvalidMove));
        }
        if to.can_add(&card) && from.can_remove(&card) {
            let card = match from.remove_card(&card) {
                Some(c) => c,
                None => return Err(Box::new(Error::InvalidMove)),
            };
            to.add_card(card);
            if from.top_card_is_covered() {
                from.flip_top_card();
            }
            Ok(())
        } else {
            Err(Box::new(Error::InvalidMove))
        }
    }

    pub fn move_cards(&self, number: usize, from: PileType, to: PileType) -> Result<()> {
        if number == 0 {
            return Err(Box::new(Error::InvalidMove));
        }
        let from = self.get_pile(from);
        let to = self.get_pile(to);
        if number == 1 {
            let card = *from.borrow().top_card().ok_or(Error::EmptyPile)?;
            return self.move_card(card, Rc::clone(&from), Rc::clone(&to));
        }
        let cards = from.borrow().get_cards(number).clone();
        let results = cards
            .into_iter()
            .map(|card| self.move_card(card, Rc::clone(&from), Rc::clone(&to)))
            .collect::<Vec<Result<()>>>();
        for res in results {
            if res.is_err() {
                return res;
            } else {
                continue;
            }
        }
        Ok(())
    }

    pub fn draw_card(&self) -> Result<()> {
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
                draw_pile.reverse();
                draw_pile.flip_all_cards();
                Ok(())
            }
        }
    }

    fn is_move_valid(&self, number: usize, from: PileRef, to: PileRef) -> bool {
        if number == 0 {
            return false;
        }
        let from = from.borrow();
        match from.card(from.length() - number) {
            Some(card) => to.borrow().can_add(card),
            None => true,
        }
    }

    pub fn auto_move(&self, from: PileType, to: PileType) -> Result<()> {
        let from_type = from;
        let to_type = to;
        let from = self.get_pile(from_type);
        let to = self.get_pile(to_type);
        if from.borrow().pile_type() == PileType::Uncovered {
            let card = *from.borrow().top_card().ok_or(Error::EmptyPile)?;
            return self.move_card(card, from, to);
        }
        let indexes = (1..=from.borrow().length())
            .map(|n| self.is_move_valid(n, Rc::clone(&from), Rc::clone(&to)))
            .enumerate()
            .map(|(i, v)| if v { (i + 1) as u8 } else { 0_u8 })
            .filter(|i| *i != 0)
            .collect::<Vec<u8>>();
        let index = *indexes.first().unwrap_or(&0);
        self.move_cards(index as usize, from_type, to_type)
    }
}

impl Default for Table {
    fn default() -> Self {
        Table::new()
    }
}

impl Clone for Table {
    fn clone(&self) -> Self {
        let draw_pile = Rc::new(RefCell::new(self.draw_pile.borrow().clone()));
        let uncovered_pile = Rc::new(RefCell::new(self.uncovered_pile.borrow().clone()));
        let lanes = self
            .lanes
            .iter()
            .map(|lane| Rc::new(RefCell::new(lane.borrow().clone())))
            .collect();

        let mut suit_piles = HashMap::new();
        for pile in self.suit_piles.values() {
            if let PileType::Suit(suit) = pile.borrow().pile_type() {
                suit_piles.insert(suit, Rc::new(RefCell::new(pile.borrow().clone())));
            }
        }
        Table {
            suit_piles,
            lanes,
            draw_pile,
            uncovered_pile,
        }
    }
}
