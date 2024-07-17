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

#[derive(Copy, Clone)]
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
}