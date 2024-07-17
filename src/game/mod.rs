use rand::Rng;
use rand::prelude::ThreadRng;
use scoped_state::ScopedGameState;

use crate::player::Player;
use crate::card::{Card, Suit};
use crate::utils::{Action, FlippedChoice};
use crate::dealer::Dealer;
use crate::judger::Judger;

pub mod scoped_state;

pub struct EuchreGame {
    is_over: bool,
    players: Vec<Player>,
    curr_player_id: u8,
    dealer_id: u8,
    dealer: Dealer,
    judger: Judger,

    flipped_card: Card,
    flipped_choice: Option<FlippedChoice>,
    calling_player_id: Option<u8>,
    previous_played: Vec<Vec<Card>>,
    center: Option<Vec<Card>>,
    order: Vec<u8>,
    trump: Option<Suit>,
    led_suit: Option<Suit>,
}

impl EuchreGame {

    /// Sets up a new EuchreGame.
    pub fn new(
            dealer_id: Option<u8>) -> EuchreGame {

        let deal_id = determine_dealer(dealer_id);
        let curr_p_id = (deal_id + 1) % 4;
        let mut dealer: Dealer = Dealer::new();

        let mut players: Vec<Player> = vec![Player::new(0), 
                                        Player::new(1),
                                        Player::new(2),
                                        Player::new(3)];
        
        for player in &mut players {
            dealer.deal_cards(player);
        }

        EuchreGame {
            is_over: false,
            players: players,
            dealer: Dealer::new(),
            curr_player_id: curr_p_id,
            dealer_id: deal_id,
            judger: Judger::new(),

            flipped_card: dealer.flip_top_card(),
            flipped_choice: None,
            calling_player_id: None,
            previous_played: vec![vec![], vec![], vec![], vec![]],
            center: None,
            order: Self::order_starting_from(curr_p_id),
            trump: None,
            led_suit: None,
        }
    }

    pub fn is_over(&self) -> bool {
        self.is_over
    }

    /// Get the current game state as the current player sees it.
    pub fn get_state(&self) -> ScopedGameState {
        ScopedGameState {
            current_actor: self.curr_player_id,
            hand: self.player_ref(self.curr_player_id).hand_ref(),
            calling_actor: self.calling_player_id,
            dealer_actor: self.dealer_id,
            flipped_card: self.flipped_card,
            flipped_choice: self.flipped_choice,
            trump: self.trump,
            led_suit: self.led_suit,

            order: &self.order,
            center: &self.center,
            previous_played: &self.previous_played,
        }
    }

    /// Update the game state based on the passed in `Action`.
    /// Returns the scoped game state and the current player's id
    pub fn step(&mut self, action: Action) -> (ScopedGameState, u8){
        if action == Action::Pick { // Pick
            self.perform_pick_action();
        } else if action == Action::Pass { // Pass
            self.perform_pass_action();
        } else if (action as u8) < 6 { // Call

        } else if (action as u8) > 29 { // Discard

        } else {

        // Play
    
            if self.center.as_ref().unwrap().len() == 4 {

            }

        }
        let state: ScopedGameState = self.get_state();
        (state, self.curr_player_id)
    }

    /// Return the id of the current player
    pub fn get_curr_player_id(&self) -> u8 {
        self.curr_player_id
    }

    // player MUST be either 0, 1, 2, or 3
    fn order_starting_from(player: u8) -> Vec<u8> {
        vec![player, (player + 1)%4, (player+2)%4, (player+3)%4]
    }

    /// Calculate and return the id of the player whose
    /// turn is next, after current_player
    fn see_next_player(&self) -> u8 {
        (self.curr_player_id + 1) % 4
    }

    /// updates `current_player_id` to hold id of player
    /// who sits left of old current player.
    fn increment_player(&mut self) {
        self.curr_player_id = self.see_next_player();
    }

    fn player_ref(&self, id: u8) -> &Player {
        &self.players[usize::from(id)]
    }

    /// Changes game state to reflect taking the `Pick` action.  
    /// 1. Dealer has `flipped_card` added to hand
    /// 2. Trump is set
    /// 3. Current player is changed to be the dealer
    /// 4. `flipped_choice` is set to `PickedUp`
    /// 5. The player who ordered up trump is recorded
    fn perform_pick_action(&mut self) {
        let dealer_player = self.player_ref(self.dealer_id);
        dealer_player.hand_ref().push(self.flipped_card);
        self.trump = Some(self.flipped_card.suit());
        self.flipped_choice = Some(FlippedChoice::PickedUp);
        self.calling_player_id = Some(self.curr_player_id);
        self.curr_player_id = self.dealer_id;
    } 

    /// Changes game state to reflect taking the `Pass` action.
    /// 1. Increment current player id by 1
    /// 2. If the player was the dealer, turn down the flipped card.
    fn perform_pass_action(&mut self) {
        if self.curr_player_id == self.dealer_id {
            self.flipped_choice = Some(FlippedChoice::TurnedDown);
        }
        self.increment_player();
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
        let game: EuchreGame = EuchreGame::new(Some(0));
    
        assert!(!game.is_over())
    }

    #[test]
    fn get_dealer_valid_dealer() {
        assert_eq!(2, determine_dealer(Some(2)));
        assert!(determine_dealer(None) < 4);
    }

    #[test]
    #[should_panic]
    fn get_dealer_panic_on_invalid_dealer() {
        determine_dealer(Some(4));
    }
}