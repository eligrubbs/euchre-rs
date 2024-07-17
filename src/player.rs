use crate::card::Card;

pub struct Player {
    player_id: u8,
    tricks: u8,
    hand: Vec<Card>,
}

impl Player {
    pub fn new(p_id: u8) -> Player {
        Player {
            player_id: p_id,
            tricks: 0,
            hand: vec![],
        }
    }

    pub fn add_cards(&mut self, cards: &mut Vec<Card>) {
        self.hand.append(cards)
    }

    pub fn award_trick(&mut self) {
        self.tricks += 1;
    }

    pub fn get_tricks(&self) -> u8 {
        self.tricks
    }

    pub fn get_id(&self) -> u8 {
        self.player_id
    }

    /// Return clone of players hand
    pub fn hand_ref(&self) -> Vec<Card> {
        let mut clone: Vec<Card> = vec![];
        clone.clone_from_slice(&self.hand[0..]);
        clone
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_player() {
        let play: Player = Player::new(2);
        let real_id: u8 = 2;
        assert_eq!(play.player_id, real_id)
    }

    #[test]
    fn award_trick() {
        let mut play: Player = Player::new(2);
        play.award_trick();
        assert_eq!(1, play.tricks);
    }

    #[test]
    fn get_tricks_won() {
        let mut play: Player = Player::new(3);
        assert_eq!(play.get_tricks(), 0);
        play.tricks = 10;
        assert_eq!(play.get_tricks(), 10);
    }

    #[test]
    fn get_id() {
        let play: Player = Player::new(3);
        assert_eq!(play.get_id(), 3);
    }
}