use crate::pile::Pile;
use core::{Game, table::Table};
use std::cell::Ref;
use strum::IntoEnumIterator;

pub mod core;
pub mod deck;
pub mod pile;

const HELP_SCREEN: &'static str = "Controls:\n
    A                   Automatically finish the game if all cards are uncovered\n
    ?                   Print help\n
    Q | Esc             Quit game\n
    L                   Print move history\n
    [Pile1][Pile2]      Automatically move cards from Pile1 to Pile2\n
    N                   Draw a card from the uncovered pile\n
    U | Backspace       Undo last move\n
    Pile can be any among 1-7, P, C, D, H, S";

pub fn setup() -> Game {
    Game::new()
}

pub fn print_table(table: &Table) {
    println!(" N\t P\t\t C\u{2663}\t D\u{2666}\t H\u{2665}\t S\u{2660}");
    print!("{}\t{}\t\t", table.draw_pile(), table.uncovered_pile());
    let suit_piles = deck::card::Suit::iter().map(|suit| table.suit_pile(suit));
    suit_piles.for_each(|suit_pile| print!("{}\t", suit_pile));
    println!("\n");

    let lanes = (0..7)
        .map(|i| table.lane(i))
        .collect::<Vec<Ref<'_, Pile>>>();
    lanes.iter().for_each(|lane| print!(" {}\t", lane));
    println!();
    let lines = lanes
        .iter()
        .max_by(|l1, l2| l1.length().cmp(&l2.length()))
        .unwrap()
        .length();
    for i in 0..lines {
        for lane in &lanes[..] {
            let card = match lane.card(i) {
                Some(c) => format!("{}\t", c),
                None => "\t".to_string(),
            };
            print!("{card}");
        }
        println!()
    }
}

pub fn print_help() {
    println!("{}", HELP_SCREEN);
}
