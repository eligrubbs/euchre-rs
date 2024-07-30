use rand_chacha::ChaCha8Rng;
use strum::IntoEnumIterator;

use crate::card::{Card, Suit, Rank};
use crate::player::Player;
use rand::seq::SliceRandom;

pub struct Dealer<'a> {
    euchre_deck: Vec<Card>,
    gen: &'a mut ChaCha8Rng,
}

impl<'a> Dealer<'a> {
    pub fn new (gen: &'a mut ChaCha8Rng) -> Dealer {
        Dealer {
            euchre_deck: Self::init_euchre_deck(),
            gen: gen,
        }
    }

    pub fn shuffle(&mut self) {
        self.euchre_deck.shuffle(&mut self.gen);
    }

    pub fn deal_cards(&mut self, player: &mut Player) {
        let at: usize = self.euchre_deck.len() - 5;
        player.add_cards(&mut self.euchre_deck.split_off(at));
    }

    pub fn flip_top_card(&mut self) -> Card {
        self.euchre_deck.pop().unwrap()
    }

    fn init_euchre_deck() -> Vec<Card> {
        let mut result:Vec<Card> = vec![];
        for d_suit in Suit::iter() {
            for d_rank in Rank::iter() {
                result.push(Card::new(d_suit, d_rank));
            }
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn init_dealer() {

        let mut gen: ChaCha8Rng = ChaCha8Rng::seed_from_u64(1);
        let mut dealer: Dealer = Dealer::new(&mut gen);

        assert_eq!(dealer.flip_top_card(), Card::new(Suit::Spades, Rank::Ace));
    }

    #[test]
    fn shuffle_is_rdm() {
        let mut gen: ChaCha8Rng = ChaCha8Rng::seed_from_u64(1);
        let mut dealer: Dealer = Dealer::new(&mut gen);

        dealer.shuffle();
        assert_eq!(dealer.flip_top_card(), Card::new(Suit::Clubs, Rank::Ten));
    }
}
