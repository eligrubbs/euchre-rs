
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

    /// Judge which team won the current round of euchre.
    /// Assumes that the sum of all elements in `tricks` is 5 (one for each trick)
    pub fn judge_round(&self, tricks: Vec<u8>, calling_id: u8) -> Vec<u8> {
        let team_1_tricks: u8 = tricks[0] + tricks[2];
        let team_1_called: bool = if calling_id % 2 == 0 {true} else {false};

        if team_1_tricks == 5 { //team 1 won all the tricks
            vec![2,0,2,0]
        } else if team_1_tricks >= 3 {
            if team_1_called { // team 1 called & won
                vec![1,0,1,0]
            } else { // team 1 euchered team 2
                vec![2,0,1,0]
            }
        } else if team_1_tricks < 2 {
            if team_1_called { // team 2 euchered team 1
                vec![0,2,0,2]
            } else { // team 2 called & won
                vec![0,1,0,1]
            }
        } else { // team 2 won all the tricks
            vec![0,2,0,2]
        }
    }
}