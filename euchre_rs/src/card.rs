

#[derive(Copy, Clone)]
pub enum Rank {
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

#[derive(Copy, Clone)]
pub enum Suit {
    Diamonds,
    Hearts,
    Clubs,
    Spades
}

pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card {suit: suit, rank: rank}
    }

    pub fn suit(&self) -> Suit {
        self.suit.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_card() {
        let card: Card = Card::new(Suit::Hearts, Rank::Ten);
        assert_eq(Suit::Hearts)
    }
}