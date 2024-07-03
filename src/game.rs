use std::collections::HashMap;
use std::hash::Hash;

use rand::Rng;
use rand::prelude::ThreadRng;

use crate::card::Card;
use crate::card::Suit;
use crate::deck::Deck;
use crate::player::{Player, Strategy};

pub struct EuchreGame {
    payoffs: Option<(u8,u8,u8,u8)>,
    players: Vec<Player>,
    dealer_id: u8,
    flipped_card: Card,
    calling_player: Option<u8>,
    flipped_choice: Option<FlippedChoice>,
    previous_played: HashMap<u8, Vec<Card>>,
    center: Option<Vec<Card>>,
    order: Vec<u8>,
    player_tricks: HashMap<u8, u8>,
    game_over: bool,
}


impl EuchreGame {
    pub fn new(&self) -> EuchreGame {

        let mut deck: Deck = Deck::new();
        deck.shuffle();

        let players: Vec<Player> = vec![
            Player::new(Strategy::Human,  deck.deal_n_cards(5), 0),
            Player::new(Strategy::Random,  deck.deal_n_cards(5), 1),
            Player::new(Strategy::Random,  deck.deal_n_cards(5), 2),
            Player::new(Strategy::Random,  deck.deal_n_cards(5), 3),
        ];

        let dealer_ind: u8 = determine_dealer();

        let flipped_card: Card = deck.deal_n_cards(1).remove(0);

        let mut player_previously: HashMap<u8, Vec<Card>> = HashMap::new();
        player_previously.insert(0, vec![]);
        player_previously.insert(1, vec![]);
        player_previously.insert(2, vec![]);
        player_previously.insert(3, vec![]);

        let mut p_tricks: HashMap<u8, u8> = HashMap::new();
        p_tricks.insert(0,0);
        p_tricks.insert(1,0);
        p_tricks.insert(2,0);
        p_tricks.insert(3,0);

        EuchreGame {
            payoffs: None,
            players: players,
            dealer_id: dealer_ind,
            flipped_card: flipped_card,
            calling_player: None,
            flipped_choice: None,
            previous_played: player_previously,
            center: None,
            order: Self::order_starting_from((dealer_ind + 1)%4),
            player_tricks: p_tricks,
            game_over: false,
        }
    }

    // player MUST be either 0, 1, 2, or 3
    fn order_starting_from(player: u8) -> Vec<u8> {
        vec![player, (player + 1)%4, (player+2)%4, (player+3)%4]
    }

}

fn determine_dealer() -> u8 {
    let mut rng: ThreadRng = rand::thread_rng();
    rng.gen_range(0..=3)
}

/// A struct which represents the game state from the perspective of a certain player
pub struct ScopedGameState {
    current_actor: u8,
    hand: Vec<Card>,
    dealer_actor: u8,
    flipped_card: Card,
    // each round has a distinct order players make decisions in.
    order: Vec<u8>,
    // All empty if not in 2nd round; cards are only in center during 1st.
    previous_played: (Vec<Card>, Vec<Card>, Vec<Card>, Vec<Card>),

    // None if trump not yet decided
    trump: Option<Suit>,
    // None if no choice on trump has been made
    calling_actor: Option<u8>,
    // None if no choice on trump has been made... Flipped card still facing up
    flipped_choice: Option<FlippedChoice>,

    // None if still deciding trump or starting new round
    led_suit: Option<Suit>,
    // None if no cards played yet, or in rounds 0/1
    center: Option<Vec<Card>>,

}

pub enum FlippedChoice {
    PickedUp,
    TurnedDown,
}

pub struct Played(u8, Card);


pub enum Action {
    Pass,
    Pick,
    CallHearts,
    CallDiamonds,
    CallSpades,
    CallClubs,
    HAPlay,
    HKPlay,
    HQPlay,
    HJPlay,
    HTPlay,
    H9Play,
    DAPlay,
    DKPlay,
    DQPlay,
    DJPlay,
    DTPlay,
    D9Play,
    SAPlay,
    SKPlay,
    SQPlay,
    SJPlay,
    STPlay,
    S9Play,
    CAPlay,
    CKPlay,
    CQPlay,
    CJPlay,
    CTPlay,
    C9Play,
    HADiscard,
    HKDiscard,
    HQDiscard,
    HJDiscard,
    HTDiscard,
    H9Discard,
    DADiscard,
    DKDiscard,
    DQDiscard,
    DJDiscard,
    DTDiscard,
    D9Discard,
    SADiscard,
    SKDiscard,
    SQDiscard,
    SJDiscard,
    STDiscard,
    S9Discard,
    CADiscard,
    CKDiscard,
    CQDiscard,
    CJDiscard,
    CTDiscard,
    C9Discard,
}