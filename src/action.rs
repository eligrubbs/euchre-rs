use strum::IntoEnumIterator;
use strum_macros::{EnumIter, Display, EnumString};

use crate::card::{Card, Suit, Rank};

#[derive(Debug, Eq, PartialEq, PartialOrd, Copy, Clone, EnumIter, Display, EnumString)]
pub enum Action {
    #[strum(ascii_case_insensitive)]
    Pass = 0,
    #[strum(ascii_case_insensitive)]
    Pick = 1,
    #[strum(ascii_case_insensitive)]
    CallH = 2,
    #[strum(ascii_case_insensitive)]
    CallD = 3,
    #[strum(ascii_case_insensitive)]
    CallS = 4,
    #[strum(ascii_case_insensitive)]
    CallC = 5,
    #[strum(ascii_case_insensitive)]
    HAPlay = 6,
    #[strum(ascii_case_insensitive)]
    HKPlay = 7,
    #[strum(ascii_case_insensitive)]
    HQPlay = 8,
    #[strum(ascii_case_insensitive)]
    HJPlay = 9,
    #[strum(ascii_case_insensitive)]
    HTPlay = 10,
    #[strum(ascii_case_insensitive)]
    H9Play = 11,
    #[strum(ascii_case_insensitive)]
    DAPlay = 12,
    #[strum(ascii_case_insensitive)]
    DKPlay = 13,
    #[strum(ascii_case_insensitive)]
    DQPlay = 14,
    #[strum(ascii_case_insensitive)]
    DJPlay = 15,
    #[strum(ascii_case_insensitive)]
    DTPlay = 16,
    #[strum(ascii_case_insensitive)]
    D9Play = 17,
    #[strum(ascii_case_insensitive)]
    SAPlay = 18,
    #[strum(ascii_case_insensitive)]
    SKPlay = 19,
    #[strum(ascii_case_insensitive)]
    SQPlay = 20,
    #[strum(ascii_case_insensitive)]
    SJPlay = 21,
    #[strum(ascii_case_insensitive)]
    STPlay = 22,
    #[strum(ascii_case_insensitive)]
    S9Play = 23,
    #[strum(ascii_case_insensitive)]
    CAPlay = 24,
    #[strum(ascii_case_insensitive)]
    CKPlay = 25,
    #[strum(ascii_case_insensitive)]
    CQPlay = 26,
    #[strum(ascii_case_insensitive)]
    CJPlay = 27,
    #[strum(ascii_case_insensitive)]
    CTPlay = 28,
    #[strum(ascii_case_insensitive)]
    C9Play = 29,
    #[strum(ascii_case_insensitive)]
    HADiscard = 30,
    #[strum(ascii_case_insensitive)]
    HKDiscard = 31,
    #[strum(ascii_case_insensitive)]
    HQDiscard = 32,
    #[strum(ascii_case_insensitive)]
    HJDiscard = 33,
    #[strum(ascii_case_insensitive)]
    HTDiscard = 34,
    #[strum(ascii_case_insensitive)]
    H9Discard = 35,
    #[strum(ascii_case_insensitive)]
    DADiscard = 36,
    #[strum(ascii_case_insensitive)]
    DKDiscard = 37,
    #[strum(ascii_case_insensitive)]
    DQDiscard = 38,
    #[strum(ascii_case_insensitive)]
    DJDiscard = 39,
    #[strum(ascii_case_insensitive)]
    DTDiscard = 40,
    #[strum(ascii_case_insensitive)]
    D9Discard = 41,
    #[strum(ascii_case_insensitive)]
    SADiscard = 42,
    #[strum(ascii_case_insensitive)]
    SKDiscard = 43,
    #[strum(ascii_case_insensitive)]
    SQDiscard = 44,
    #[strum(ascii_case_insensitive)]
    SJDiscard = 45,
    #[strum(ascii_case_insensitive)]
    STDiscard = 46,
    #[strum(ascii_case_insensitive)]
    S9Discard = 47,
    #[strum(ascii_case_insensitive)]
    CADiscard = 48,
    #[strum(ascii_case_insensitive)]
    CKDiscard = 49,
    #[strum(ascii_case_insensitive)]
    CQDiscard = 50,
    #[strum(ascii_case_insensitive)]
    CJDiscard = 51,
    #[strum(ascii_case_insensitive)]
    CTDiscard = 52,
    #[strum(ascii_case_insensitive)]
    C9Discard = 53,
}

impl Action {
    /// Map all \<Card\>Play and \<Card\>Discard actions to the \<Card\> they refer to.
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
            Suit::Unset => panic!("Unset Suit should be impossible here."),
        };
        let rank_num: u8 = match card.rank() {
            Rank::Ace => 0,
            Rank::King => 1,
            Rank::Queen => 2,
            Rank::Jack => 3,
            Rank::Ten => 4,
            Rank::Nine => 5,
            Rank::Unset => panic!("Unset Rank should be impossible here."),
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


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FlippedChoice {
    PickedUp,
    TurnedDown,
}



#[cfg(test)]
mod tests {
    use std::str::FromStr;

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

    #[test]
    fn string_to_action() {
        assert_eq!(Action::C9Play, Action::from_str("c9PlAy").unwrap());
    }
    
}
