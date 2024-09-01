use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, EnumIter)]
pub enum Rank {
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
    /// Reserved for initializing decks, indicates that this card can be given any Rank
    Unset = 0,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum Suit {
    Diamonds,
    Hearts,
    Clubs,
    Spades,
    /// Reserved for initializng decks, indicates that this card can be given any Suit
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
}