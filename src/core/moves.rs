use crate::deck::card::Suit;
use crate::pile::PileType;

#[derive(Debug)]
pub enum Move {
    DrawCard,
    AutoMove(PileType, PileType),
    MoveCards(usize, PileType, PileType),
    History,
    Help,
    Undo,
    Quit,
    Invalid,
}
impl Move {
    fn parse_piles(piles: &str) -> Vec<Option<PileType>> {
        piles
            .chars()
            .map(|pile| match pile {
                'P' => Some(PileType::Uncovered),
                'C' => Some(PileType::Suit(Suit::Clubs)),
                'D' => Some(PileType::Suit(Suit::Diamonds)),
                'H' => Some(PileType::Suit(Suit::Hearts)),
                'S' => Some(PileType::Suit(Suit::Spades)),
                c if c.is_numeric() && (1..=7).contains(&c.to_digit(10).unwrap()) => {
                    Some(PileType::Lane(c.to_digit(10).unwrap() as usize))
                }
                _ => None,
            })
            .collect()
    }
}

impl From<String> for Move {
    fn from(item: String) -> Move {
        let item = item.trim().to_uppercase();
        let m = match item.len() {
            1 => match &item[..] {
                "D" => Move::DrawCard,
                "H" => Move::Help,
                "L" => Move::History,
                "Q" => Move::Quit,
                "U" => Move::Undo,
                _ => Move::Invalid,
            },
            2 => match &item[..] {
                piles => {
                    let ps = Move::parse_piles(piles);
                    match &ps[..] {
                        [Some(a), Some(b)] => Move::AutoMove(*a, *b),
                        _ => Move::Invalid,
                    }
                }
            },
            3 | 4 => match (&item[..2], &item[2..]) {
                (piles, number) => {
                    let n: usize = match number.parse() {
                        Ok(result) => result,
                        Err(_) => 0,
                    };
                    let ps = Move::parse_piles(piles);
                    match &ps[..] {
                        [Some(a), Some(b)] => Move::MoveCards(n, *a, *b),
                        _ => Move::Invalid,
                    }
                }
            },
            _ => Move::Invalid,
        };
        println!("{:?}", m);
        m
    }
}
