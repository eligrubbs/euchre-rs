use std::default;

use rand::Rng;
use rand::prelude::ThreadRng;

use crate::player::Player;
use crate::utils::Action;

pub struct EuchreGame {
    is_over: bool,
    players: (Player, Player, Player, Player),
    dealer_id: u8,
}

impl EuchreGame {

    pub fn new(is_over: bool,
            players: (Player, Player, Player, Player),
            dealer_id: Option<u8>) -> EuchreGame {

        EuchreGame {
            is_over: is_over,
            players: players,
            dealer_id: determine_dealer(dealer_id),
        }
    }

    pub fn is_over(&self) -> bool {
        self.is_over
    }

    /// Update the game state based on the passed in `Action`.
    pub fn step(&self, action: Action) {
        if action == Action::Pick {

        }
    }

}

/// Either return the passed in `Some(u8)`, panicing if greater than 3, 
/// or randomly choose a number on the range \[0,3\]
fn determine_dealer(deal_id: Option<u8>) -> u8 {

    if let Some(num) = deal_id {
        assert!(num < 4);
        return num
    }

    let mut rng: ThreadRng = rand::thread_rng();
    rng.gen_range(0..=3)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_game() {
        let players: (Player, Player, Player, Player) = (Player::new(0), 
                                                         Player::new(1),
                                                         Player::new(2),
                                                         Player::new(3));

        let game: EuchreGame = EuchreGame::new(false, players, Some(0));
    
        assert!(!game.is_over())
    }

    #[test]
    fn det_dealer_valid_dealer() {
        assert_eq!(2, determine_dealer(Some(2)));
        assert!(determine_dealer(None) < 3);
    }

    #[test]
    #[should_panic]
    fn det_dealer_panic_on_invalid_dealer() {
        determine_dealer(Some(4));
    }
}