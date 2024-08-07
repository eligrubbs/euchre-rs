
use crate::card::{Card, Suit};
use crate::utils::FlippedChoice;
use crate::utils::Action;

#[derive(Debug)]
/// A struct which represents the game state from the perspective of a certain player
pub struct ScopedGameState {
    pub dealer_actor: u8,
    pub current_actor: u8,
    /// Clone of current players hand
    pub hand: Vec<Card>,

    /// None if no choice on trump has been made
    pub calling_actor: Option<u8>,
    /// None if no choice on trump has been made... Flipped card still facing up
    pub flipped_choice: Option<FlippedChoice>,

    pub flipped_card: Card,
    
    /// None if trump not yet decided
    pub trump: Option<Suit>,

    /// None if still deciding trump or starting new round
    pub led_suit: Option<Suit>,
    
    /// each round has a distinct order players make decisions in.
    pub order: Vec<u8>,
    /// None if no cards played yet, or in rounds 0/1
    pub center: Option<Vec<Card>>,
    /// All empty if not in 2nd round; cards are only in center during 1st.
    pub previous_played: Vec<Vec<Card>>,

    /// Guranteed to always exist, only empty after last turn of game.
    pub legal_actions: Vec<Action>,

}