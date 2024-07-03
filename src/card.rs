use std::slice::Iter;
use std::fmt;
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
#[derive(Clone)]
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


#[derive(Clone)]
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