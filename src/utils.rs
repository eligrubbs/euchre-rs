use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::card::{Card, Suit, Rank};

#[derive(Debug, Eq, PartialEq, PartialOrd, Copy, Clone, EnumIter)]
pub enum Action {
    Pass = 0,
    Pick = 1,
    CallH = 2,
    CallD = 3,
    CallS = 4,
    CallC = 5,
    HAPlay = 6,
    HKPlay = 7,
    HQPlay = 8,
    HJPlay = 9,
    HTPlay = 10,
    H9Play = 11,
    DAPlay = 12,
    DKPlay = 13,
    DQPlay = 14,
    DJPlay = 15,
    DTPlay = 16,
    D9Play = 17,
    SAPlay = 18,
    SKPlay = 19,
    SQPlay = 20,
    SJPlay = 21,
    STPlay = 22,
    S9Play = 23,
    CAPlay = 24,
    CKPlay = 25,
    CQPlay = 26,
    CJPlay = 27,
    CTPlay = 28,
    C9Play = 29,
    HADiscard = 30,
    HKDiscard = 31,
    HQDiscard = 32,
    HJDiscard = 33,
    HTDiscard = 34,
    H9Discard = 35,
    DADiscard = 36,
    DKDiscard = 37,
    DQDiscard = 38,
    DJDiscard = 39,
    DTDiscard = 40,
    D9Discard = 41,
    SADiscard = 42,
    SKDiscard = 43,
    SQDiscard = 44,
    SJDiscard = 45,
    STDiscard = 46,
    S9Discard = 47,
    CADiscard = 48,
    CKDiscard = 49,
    CQDiscard = 50,
    CJDiscard = 51,
    CTDiscard = 52,
    C9Discard = 53,
}

impl Action {
    /// Map all <Card>Play and <Card>Discard actions to the <Card> they refer to.
    pub fn action_to_card(action: Action) -> Result<Card, String> {
        // the number % 6 returns the rank
        // 0 1 2 3 4 5
        // A K Q J T 9
        // the ((number % 30) + 6(only if a Discard) ) / 6 gives suit
        // 1 2 3 4
        // H D S C
        let num: u8 = action as u8;
        if num < 6 {
            return Err(format!("The action {:?} has no mapping to a Card", action))
        }
        let num_rank: u8 = num % 6;
        let num_suit: u8 = (((if num > 29 {num + 6} else {num}) % 30)) / 6;
        
        let rank: Rank = match num_rank {
            0 => Rank::Ace,
            1 => Rank::King,
            2 => Rank::Queen,
            3 => Rank::Jack,
            4 => Rank::Ten,
            5 => Rank::Nine,
            _ => panic!("Action space mapping to rank did not land between 0-5"),
        };

        let suit: Suit = match num_suit {
            1 => Suit:: Hearts,
            2 => Suit::Diamonds,
            3 => Suit::Spades,
            4 => Suit::Clubs,
            _ => panic!("Action space mapping to suit did not land between 1-4"),
        };

        Ok(Card::new(suit, rank))
    }

    /// Map a Card to either a play or discard action.
    pub fn card_to_action(card: &Card, is_play: bool) -> Action {
        let suit_num: u8 = match card.suit() {
            Suit::Hearts => 1,
            Suit::Diamonds => 2,
            Suit::Spades => 3,
            Suit::Clubs => 4,
        };
        let rank_num: u8 = match card.rank() {
            Rank::Ace => 0,
            Rank::King => 1,
            Rank::Queen => 2,
            Rank::Jack => 3,
            Rank::Ten => 4,
            Rank::Nine => 5,
        };
        let discard_offset: u8 = if is_play {0} else {24};
        let num: u8 = rank_num + (suit_num * 6) + discard_offset;
        Action::from_integer(num).unwrap()
    }

    pub fn from_integer(val: u8) -> Result<Action, String> {
        for act in Action::iter() {
            if act as u8 == val {
                return Ok(act)
            }
        }
        Err(format!("Integer {:?} has no corresponding Action.", val))
    }
}


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FlippedChoice {
    PickedUp,
    TurnedDown,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_from_int() {
        assert_eq!(Action::from_integer(13).unwrap(), Action::DKPlay);
        assert_eq!(Action::from_integer(0).unwrap(), Action::Pass);
    }

    #[test]
    fn action_from_invalid_int() {
        assert_eq!(Action::from_integer(54).unwrap_err(), "Integer 54 has no corresponding Action.");
    }

    #[test]
    fn action_to_card() {
        assert_eq!(Action::action_to_card(Action::C9Discard).unwrap(),
        Action::action_to_card(Action::C9Play).unwrap());

        assert_eq!(Action::action_to_card(Action::HADiscard).unwrap(),
        Action::action_to_card(Action::HAPlay).unwrap());
    }
}
