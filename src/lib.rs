use crate::pile::Pile;
use core::{Game, table::Table};
use std::cell::Ref;
use strum::IntoEnumIterator;

pub mod core;
pub mod deck;
pub mod pile;

pub fn setup() -> Game {
    Game::new()
}

pub fn print_table(table: &Table) {
    println!(" D\t P\t\t C\u{2663}\t D\u{2666}\t H\u{2665}\t S\u{2660}");
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
    println!(
        "Controls:\n
    H                   Print help\n
    Q                   Quit game\n
    L                   Print move history\n
    [Pile1][Pile2]      Automatically move cards from Pile1 to Pile2\n
    [Pile1][Pile2][n]   Move n cards from Pile1 to Pile2\n
    D                   Draw a card from the uncovered pile\n
    U                   Undo last move\n
    Pile can be any between 1-7, P, C, D, H, S"
    );
}
