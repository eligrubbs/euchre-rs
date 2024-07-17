use crate::card::Card;

pub struct Player {
    player_id: u8,
    hand: Vec<Card>,
}

impl Player {
    pub fn new(p_id: u8) -> Player {
        Player {
            player_id: p_id,
            hand: vec![],
        }
    }

    pub fn add_cards(&mut self, cards: &mut Vec<Card>) {
        self.hand.append(cards)
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
}