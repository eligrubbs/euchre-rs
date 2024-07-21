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
    scores: Option<Vec<u8>>,
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
            scores: None,
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
    pub fn get_state(&mut self) -> ScopedGameState {
        let legals: Vec<Action> = self.get_legal_actions();

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
            legal_actions: legals,
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
            self.perform_call_action(action);
        } else if (action as u8) > 29 { // Discard
            self.perform_discard_action(action);
        } else {// Play
            self.perform_play_card(action);

            if self.center.as_ref().unwrap().len() == 4 {
                self.end_trick();
                self.decide_is_over();
            }
        }
        let curr_id: u8 = self.get_curr_player_id();
        let state: ScopedGameState = self.get_state();
        (state, curr_id)
    }

    /// Return the id of the current player
    pub fn get_curr_player_id(&self) -> u8 {
        self.curr_player_id
    }

    /// Return all actions that the current player may
    /// select based on the game state.
    pub fn get_legal_actions(&mut self) -> Vec<Action> {
        let hand: Vec<Card> = self.player_ref(self.curr_player_id).hand_ref();
        let mut actions: Vec<Action> = vec![];
        if hand.len() == 6 { // dealer must discard
            actions = hand.iter().map(|x| Action::card_to_action(x, false)).collect();
        
        } else if self.trump.is_none() { // deciding trump
            if self.flipped_choice.is_none() { // flipped_card available
                actions = vec![Action::Pick, Action::Pass];
            } else if self.flipped_choice.unwrap() == FlippedChoice::TurnedDown { // no flipped card, dealer can't pass
                let turned_down_suit: u8 = match self.flipped_card.suit() {
                    Suit::Hearts => 0,
                    Suit::Diamonds => 1,
                    Suit::Spades => 2,
                    Suit::Clubs => 3,
                };
                let mut call_suits: Vec<Action> = vec![Action::CallH, Action::CallD, Action::CallS, Action::CallC];
                call_suits.remove(usize::from(turned_down_suit));
                actions = call_suits;
                if self.get_curr_player_id() != self.dealer_id {
                    actions.push(Action::Pass);
                }
            }

        } else if self.led_suit.is_none() { // lead the trick
            actions = hand.iter().map(|x| Action::card_to_action(x, true)).collect();
        } else { // play given a led suit
            let (t_trump, t_led) = (self.trump.unwrap(), self.led_suit.unwrap());
            let t_acts: Vec<Action> = hand.iter()
                          .filter(|x| (x.suit() == t_led && !x.is_left(t_trump))
                                           || (x.suit() == t_trump && x.is_left(t_trump)))
                          .map(|x| Action::card_to_action(x, true)).collect();

            if t_acts.len() > 0 { // can follow suit
                actions = t_acts;
            } else { // can't follow suit
                actions = hand.iter().map(|x| Action::card_to_action(x, true)).collect();
            }
        }
        actions
    }

    /// Get rewards for the players
    /// Will only return not None if the game is over.
    pub fn get_rewards(&self) -> Option<Vec<u8>> {
        if self.is_over() {Some(self.scores.as_ref().unwrap().clone())} else {None}
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

    fn player_ref(&mut self, id: u8) -> &mut Player {
        &mut self.players[usize::from(id)]
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

    /// Changes game state to reflect taking a `Call` action.  
    /// 1. Sets trump to suit that is called
    /// 2. Sets current player to player left of dealer
    fn perform_call_action(&mut self, action: Action) {
        let trump: Suit = match action {
            Action::CallH => Suit::Hearts,
            Action::CallC => Suit::Clubs,
            Action::CallD => Suit::Diamonds,
            Action::CallS => Suit::Spades,
            _ => {panic!("Invalid action to perform call action: {:?}", action)}
        };
        self.trump = Some(trump);
        self.curr_player_id = (self.dealer_id + 1) % 4;
    }

    /// Changes game state to reflect taking a `Discard` action.
    /// 1. set current player to player left of dealer (also current player)
    /// 2. Remove specified card from players hand
    fn perform_discard_action(&mut self, action: Action) {
        let card_to_drop: Card = Action::action_to_card(action).unwrap();
    
        let player: &Player = self.player_ref(self.curr_player_id);
        for (i, card) in player.hand_ref().iter().enumerate() {
            if card == &card_to_drop {
                player.hand_ref().remove(i);
                break
            }
        }

        self.increment_player();
    }

    /// Changes game state to reflect taking a `Play` action.  
    /// 1. set current player to player left of current player
    /// 2. removes played card from players hand
    /// 3. adds played card to center
    /// 4. if first played card, sets led_suit
    fn perform_play_card(&mut self, action: Action) {
        let card_to_play: Card = Action::action_to_card(action).unwrap();

        let player: &Player = self.player_ref(self.curr_player_id);
        for (i, card) in player.hand_ref().iter().enumerate() {
            if card == &card_to_play {
                player.hand_ref().remove(i);
                break
            }
        }

        if self.center.is_none() {
            self.center = Some(vec![]);
            self.led_suit = 
                if card_to_play.is_left(self.trump.unwrap()) {
                    Some(self.trump.unwrap())
                } else {
                    Some(card_to_play.suit())
                };
        } else {
            self.center.as_mut().unwrap().push(card_to_play);
        }
    
        self.previous_played[usize::from(self.curr_player_id)].push(card_to_play);

        self.increment_player();
    }

    /// judges the center cards and increments players trick counts.
    /// 1. Sets current player as winner of trick.
    /// 2. Sets to None center, led_suit
    /// 3. resets order starting from winner of trick.
    fn end_trick(&mut self) {
        let winner_id = self.judger.judge_trick(
                                self.trump.unwrap(),
                                self.center.as_ref().unwrap(),
                                &self.order);
        self.player_ref(winner_id).award_trick();
        self.curr_player_id = winner_id;

        self.center = None;
        self.order = Self::order_starting_from(winner_id);
        self.led_suit = None;

    }

    /// Decide if the game is over, and if so returns points awarded to each player.
    /// Conditions: 
    /// 1. If the current player has no cards in their hand
    /// 2. Assumes that this function is only called at the end of a trick
    fn decide_is_over(&mut self) {
        if self.player_ref(self.curr_player_id).hand_ref().len() == 0 {
            self.is_over = true;
            self.scores = Some(self.judger.judge_round(
                                        self.get_player_tricks(),
                                        self.calling_player_id.unwrap()));
        }
    }

    fn get_player_tricks(&self) -> Vec<u8> {
        let mut tricks: Vec<u8> = vec![];
        for p in self.players.as_slice() {
            tricks.push(p.get_tricks());
        }
        tricks
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