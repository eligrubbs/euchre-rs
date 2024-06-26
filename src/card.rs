use std::slice::Iter;
use self::Suit::*;
use self::Rank::*;

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}


/// Idea to iterate through options from: https://stackoverflow.com/a/21376984
#[derive(Clone, Debug)]
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


#[derive(Clone, Debug)]
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