
use crate::card::{Card, Suit, Rank};

pub struct Judger {

}

impl Judger {
    pub fn new() -> Judger {
        Judger {}
    }

    /// return the id of the player that won this trick.  
    /// Assumes that the indices of center corresponds to the indices of items in `order`,
    /// and that the first index represents who led this trick.
    pub fn judge_trick(&self, trump: Suit, center: &Vec<Card>, order: &Vec<u8>) -> u8{
        let mut candidate_player: u8 = order[0];
        let mut candidate_card: Card = center[0];

        for i in 1..=3usize {
            if candidate_card.is_lower(trump, center[i]) {
                candidate_card = center[i];
                candidate_player = order[i];
            }
        }
        candidate_player
    }
}