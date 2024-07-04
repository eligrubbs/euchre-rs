use std::collections::HashMap;
use std::slice::Iter;
use std::fmt;
use std::str::FromStr;
use crate::game::Action;
use self::Suit::*;
use self::Rank::*;

#[derive(Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}


/// Idea to iterate through options from: https://stackoverflow.com/a/21376984
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds
}

impl Suit {
    pub fn iterator() -> Iter<'static, Suit> {
        static SUITS: [Suit; 4] = [Spades, Clubs, Hearts, Diamonds];
        SUITS.iter()
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Spades => write!(f, "S"),
            Suit::Clubs => write!(f, "C"),
            Suit::Hearts => write!(f, "H"),
            Suit::Diamonds => write!(f, "D")
        }
    }
}


#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Rank {
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

impl Rank {
    pub fn iterator() -> Iter<'static, Rank> {
        static RANKS: [Rank; 6] = [Nine, Ten, Jack, Queen, King, Ace];
        RANKS.iter()
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "T"),
            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
            Rank::Ace => write!(f, "A"),
        }
    }
}

pub trait EuchreCard {
    fn is_left(&self, trump: &Suit) -> bool;
    fn is_right(&self, trump: &Suit) -> bool;
    fn call_action(&self) -> Action;
    fn discard_action(&self) -> Action;
    fn play_action(&self) -> Action;
}

impl EuchreCard for Card {
    fn is_left(&self, trump: &Suit) -> bool {
        let convert: HashMap<Suit, Suit> = HashMap::from([
            (Suit::Diamonds, Suit::Hearts),
            (Suit::Hearts, Suit::Diamonds),
            (Suit::Clubs, Suit::Spades),
            (Suit::Spades, Suit::Clubs),
        ]);
        self.rank == Rank::Jack && convert.get(&self.suit).unwrap() == trump
    }

    fn is_right(&self, trump: &Suit) -> bool {
        self.suit == *trump && self.rank == Rank::Jack
    }

    fn call_action(&self) -> Action {
        Action::from_str(format!("{}Call",self.to_string()).as_str()).unwrap()
    }

    fn discard_action(&self) -> Action {
        Action::from_str(format!("{}Discard",self.to_string()).as_str()).unwrap()
    }

    fn play_action(&self) -> Action {
        Action::from_str(self.to_string().as_str()).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use std::fmt::Write;
    use super::*;


    #[test]
    fn print_suits() {
        let mut result: String = String::new();
        for suit in Suit::iterator() {
            write!(result, "{}", suit).expect("Something wrong");
        }
        assert_eq!(result, "SCHD");
    }

    #[test]
    fn print_ranks() {
        let mut result: String = String::new();
        for rank in Rank::iterator() {
            write!(result, "{}", rank).expect("Something wrong");
        }
        assert_eq!(result, "9TJQKA");
    }

    #[test]
    fn print_cards() {
        for suit in Suit::iterator() {
            for rank in Rank::iterator() {
                let card: Card = Card {suit: suit.clone(), rank: rank.clone()};
                assert_eq!(format!("{}{}", suit, rank), format!("{}", card));
            }
        }
    }
}