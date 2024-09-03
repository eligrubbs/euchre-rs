use std::fmt;

use strum_macros::{EnumIter, Display};

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, EnumIter, Display)]
pub enum Rank {
    #[strum(to_string="9")]
    Nine = 9,
    #[strum(to_string="T")]
    Ten = 10,
    #[strum(to_string="J")]
    Jack = 11,
    #[strum(to_string="Q")]
    Queen = 12,
    #[strum(to_string="K")]
    King = 13,
    #[strum(to_string="A")]
    Ace = 14,
    /// Reserved for initializing decks, indicates that this card can be given any Rank
    #[strum(to_string="?")]
    Unset = 0,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, Display)]
pub enum Suit {
    #[strum(to_string="D")]
    Diamonds,
    #[strum(to_string="H")]
    Hearts,
    #[strum(to_string="C")]
    Clubs,
    #[strum(to_string="S")]
    Spades,
    /// Reserved for initializng decks, indicates that this card can be given any Suit
    #[strum(to_string="?")]
    Unset,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    /// Creates new `Card` with specified suit and rank
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card {suit: suit, rank: rank}
    }

    /// Returns a clone of this cards `Suit`
    pub fn suit(&self) -> Suit {
        self.suit.clone()
    }

    /// Returns a clone of this cards `Rank`
    pub fn rank(&self) -> Rank {
        self.rank.clone()
    }

    /// Returns whether this card is the left bower
    /// given a trump suit
    pub fn is_left(&self, trump: Suit) -> bool {
        self == &Card::new(match trump {
            Suit::Clubs => Suit::Spades,
            Suit::Diamonds => Suit::Hearts,
            Suit::Spades => Suit::Clubs,
            Suit::Hearts => Suit::Diamonds,
            Suit::Unset => panic!("Unset Suit should be impossible here."),
        }, Rank::Jack)
    }

    /// Returns whether this card is the right bower
    /// given a trump suit
    pub fn is_right(&self, trump: Suit) -> bool {
        self == &Card::new(trump, Rank::Jack)
    }

    /// Determines if `self` is lower than other, given trump.  
    /// Assumes that `self` is the led_suit (remember the left's suit is trump).
    pub fn is_lower(&self, trump: Suit, other: Card) -> bool {
        let eff_suit: Suit = if self.is_left(trump) {trump} else {self.suit()};
        let o_eff_suit: Suit = if other.is_left(trump) {trump} else {other.suit()};

        if eff_suit == o_eff_suit {
            // If here, and both bowers are being compared, there will be an equality on rank
            // so, looking at self.is_right will break that tie
            if self.rank() > other.rank() || self.is_right(trump) {
                false
            } else {
                true
            }
        } else if o_eff_suit == trump {
            true
        } else {
            false
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_card() {
        let card: Card = Card::new(Suit::Hearts, Rank::Ten);
        assert_eq!(Suit::Hearts, card.suit());
        assert_eq!(Rank::Ten, card.rank());

    }

    #[test]
    fn is_left() {
        assert!(Card::new(Suit::Clubs, Rank::Jack).is_left(Suit::Spades));
        assert!(!Card::new(Suit::Clubs, Rank::Jack).is_left(Suit::Clubs));
        assert!(!Card::new(Suit::Clubs, Rank::Nine).is_left(Suit::Spades));
        assert!(!Card::new(Suit::Clubs, Rank::Jack).is_left(Suit::Diamonds));
    }

    #[test]
    fn is_right() {
        assert!(!Card::new(Suit::Clubs, Rank::Jack).is_right(Suit::Spades));
        assert!(Card::new(Suit::Clubs, Rank::Jack).is_right(Suit::Clubs));
        assert!(!Card::new(Suit::Clubs, Rank::Nine).is_right(Suit::Spades));
        assert!(!Card::new(Suit::Diamonds, Rank::Jack).is_right(Suit::Spades));
    }

    #[test]
    fn is_lower_for_bowers() {
        let trump: Suit = Suit::Spades;
        let o_right: Card = Card::new(Suit::Spades, Rank::Jack);
        let o_left: Card = Card::new(Suit::Clubs, Rank::Jack);
        assert!(Card::new(Suit::Diamonds, Rank::Ace).is_lower(trump, o_right));
        assert!(Card::new(Suit::Diamonds, Rank::Ace).is_lower(trump, o_left));
        assert!(o_left.is_lower(trump, o_right));
        assert!(!o_right.is_lower(trump, o_left));
    }

    #[test]
    fn card_to_string() {
        assert_eq!("??", format!("{}", Card::new(Suit::Unset, Rank::Unset)));
        assert_eq!("SA", format!("{}", Card::new(Suit::Spades, Rank::Ace)));
        assert_eq!("C9", format!("{}", Card::new(Suit::Clubs, Rank::Nine)));
    }
}