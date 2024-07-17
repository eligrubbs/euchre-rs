use std::collections::HashMap;
use strum_macros::EnumString;
use std::str::FromStr;

use rand::Rng;
use rand::prelude::ThreadRng;

use crate::card::Card;
use crate::card::Suit;
use crate::card::EuchreCard;
use crate::deck::Deck;
use crate::player::{Player, Strategy};

pub struct EuchreGame {
    payoffs: Option<(u8,u8,u8,u8)>,
    players: Vec<Player>,
    current_player: u8,
    dealer_id: u8,
    flipped_card: Card,
    calling_player: Option<u8>,
    flipped_choice: Option<FlippedChoice>,
    previous_played: (Vec<Card>, Vec<Card>, Vec<Card>, Vec<Card>),
    center: Option<Vec<Card>>,
    order: Vec<u8>,
    player_tricks: HashMap<u8, u8>,
    trump: Option<Suit>,
    led_suit: Option<Suit>,
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

        let mut p_tricks: HashMap<u8, u8> = HashMap::new();
        p_tricks.insert(0,0);
        p_tricks.insert(1,0);
        p_tricks.insert(2,0);
        p_tricks.insert(3,0);

        let curr_player = (dealer_ind + 1) % 4;

        EuchreGame {
            payoffs: None,
            players: players,
            current_player: curr_player,
            dealer_id: dealer_ind,
            flipped_card: flipped_card,
            calling_player: None,
            flipped_choice: None,
            previous_played: (vec![], vec![], vec![], vec![]),
            center: None,
            order: Self::order_starting_from(curr_player),
            player_tricks: p_tricks,
            trump: None,
            led_suit: None,
            game_over: false,
        }
    }

    // player MUST be either 0, 1, 2, or 3
    fn order_starting_from(player: u8) -> Vec<u8> {
        vec![player, (player + 1)%4, (player+2)%4, (player+3)%4]
    }

    fn gen_legal_actions(&self) -> Vec<Action> {
        let hand: &Vec<Card> = &self.players.get(usize::from(self.current_player)).unwrap().hand;

        if hand.len() == 6{
            return hand.into_iter()
            .map(|x: &Card| x.discard_action()).collect();
        } 
        
        if self.trump.is_none() {

            if self.flipped_choice.is_none() {
                return vec![Action::Pick, Action::Pass]

            } else {
                let mut actions: Vec<Action> = hand.into_iter()
                .filter(|card| card.suit != self.flipped_card.suit)
                .map(|card| card.call_action())
                .collect();

                if self.current_player != self.dealer_id {
                    actions.push(Action::Pass);
                }
                return actions
            }
        }

        if self.led_suit.is_none() {
            return hand.into_iter()
            .map(|x: &Card| x.play_action()).collect();
        }

        let follow_actions: Vec<Action> = hand.into_iter()
        .filter(|x| (x.suit == *self.led_suit.as_ref().unwrap() && !x.is_left(&self.trump.as_ref().unwrap())) ||
                            (x.is_left(&self.trump.as_ref().unwrap()) && self.led_suit == self.trump)  
               ).map(|x| x.play_action()).collect();
        if follow_actions.len() > 0{
               return follow_actions
        }

        hand.into_iter().map(|x| x.play_action()).collect()
    }

    /// Gets the current scoped game state.  
    /// 
    /// Only contains information that the current actor would know
    fn get_scoped_state(&self) -> ScopedGameState {
        ScopedGameState {
            current_actor: self.current_player,
            hand: self.players[usize::from(self.current_player)].hand.clone(),
            dealer_actor: self.dealer_id,
            flipped_card: self.flipped_card.clone(),
            order: self.order.clone(),
            previous_played: self.previous_played.clone(),
            trump: self.trump.clone(),
            calling_actor: self.calling_player.clone(),
            flipped_choice: self.flipped_choice.clone(),
            led_suit: self.led_suit.clone(),
            center: self.center.clone(),
        }
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

#[derive(Clone)]
pub enum FlippedChoice {
    PickedUp,
    TurnedDown,
}

pub struct Played(u8, Card);

// idea from: https://stackoverflow.com/a/62711168
#[derive(EnumString)]
pub enum Action {
    Pass,
    Pick,
    CallH,
    CallD,
    CallS,
    CallC,
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
