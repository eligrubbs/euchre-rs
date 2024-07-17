use strum::IntoEnumIterator;

use crate::card::{Card, Suit, Rank};
use crate::player::Player;
use rand::seq::SliceRandom;

pub struct Dealer {
    euchre_deck: Vec<Card>,
}

impl Dealer {
    pub fn new() -> Dealer {
        Dealer {
            euchre_deck: init_euchre_deck(),
        }
    }

    pub fn shuffle(&mut self) {
        self.euchre_deck.shuffle(&mut rand::thread_rng());
    }

    pub fn deal_cards(&mut self, player: &mut Player) {
        let at: usize = self.euchre_deck.len() - 5;
        player.add_cards(&mut self.euchre_deck.split_off(at));
    }

    pub fn flip_top_card(&mut self) -> Card {
        self.euchre_deck.pop().unwrap()
    }

}


fn init_euchre_deck() -> Vec<Card> {
    let mut result:Vec<Card> = vec![];
    for d_suit in Suit::iter() {
        for d_rank in Rank::iter() {
            result.push(Card::new(d_suit, d_rank));
        }
    }
    result
}
