use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum Rank {
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum Suit {
    Diamonds,
    Hearts,
    Clubs,
    Spades
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

    /// returns a clone of this cards `Suit`
    pub fn suit(&self) -> Suit {
        self.suit.clone()
    }

    /// returns a clone of this cards `Rank`
    pub fn rank(&self) -> Rank {
        self.rank.clone()
    }

    /// returns whether this card is the left bower
    /// given a trump suit
    pub fn is_left(&self, trump: Suit) -> bool {
        self == &Card::new(match trump {
            Suit::Clubs => Suit::Spades,
            Suit::Diamonds => Suit::Hearts,
            Suit::Spades => Suit::Clubs,
            Suit::Hearts => Suit::Diamonds,
        }, Rank::Jack)
    }

    /// returns whether this card is the right bower
    /// given a trump suit
    pub fn is_right(&self, trump: Suit) -> bool {
        self == &Card::new(trump, Rank::Jack)
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
}