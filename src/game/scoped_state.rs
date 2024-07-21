
use crate::card::{Card, Suit};
use crate::utils::FlippedChoice;
use crate::utils::Action;

/// A struct which represents the game state from the perspective of a certain player
pub struct ScopedGameState<'a> {
    pub current_actor: u8,
    pub hand: Vec<Card>,
    pub dealer_actor: u8,

    // None if no choice on trump has been made
    pub calling_actor: Option<u8>,
    // None if no choice on trump has been made... Flipped card still facing up
    pub flipped_choice: Option<FlippedChoice>,

    pub flipped_card: Card,
    
    // None if trump not yet decided
    pub trump: Option<Suit>,

    // None if still deciding trump or starting new round
    pub led_suit: Option<Suit>,
    
    // each round has a distinct order players make decisions in.
    pub order: &'a Vec<u8>,
    // None if no cards played yet, or in rounds 0/1
    pub center: &'a Option<Vec<Card>>,
    // All empty if not in 2nd round; cards are only in center during 1st.
    pub previous_played: &'a Vec<Vec<Card>>,

    // Should always exist
    pub legal_actions: Vec<Action>,

}