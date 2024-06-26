use crate::card::*;

#[derive(Debug)]
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

fn create_all_card_combos() -> Vec<Card> {
    let mut result:Vec<Card> = vec![];
    for d_suit in Suit::iterator() {
        for d_rank in Rank::iterator() {
            result.push(Card {suit: d_suit.clone(), rank: d_rank.clone()});
        }
    }
    result
}