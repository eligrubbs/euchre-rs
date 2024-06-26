use crate::card::*;
use std::fmt;

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Self {
        Self {
            cards: create_all_card_combos()
        }
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Deck {{ ");
        for card in &self.cards {
            write!(f, "{}, ", card);
        }
        write!(f, "}}")
    }
}

fn create_all_card_combos() -> Vec<Card> {
    let mut result:Vec<Card> = vec![];
    for d_suit in Suit::iterator() {
        for d_rank in Rank::iterator() {
            result.push(Card {suit: d_suit.clone(), rank: d_rank.clone()});
        }
    }
    result
}