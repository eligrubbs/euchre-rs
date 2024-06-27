use crate::card::*;
use std::fmt;
use rand::seq::SliceRandom;

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Self {
        Self {
            cards: create_all_card_combos()
        }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }

    pub fn deal_n_cards(&mut self, num: usize) -> Vec<Card> {
        let at: usize = self.cards.len() - num;
        self.cards.split_off(at)
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Deck {{ ").ok();
        for card in &self.cards {
            write!(f, "{}, ", card).ok();
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