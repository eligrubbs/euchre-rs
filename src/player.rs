use std::fmt;
use crate::card::Card;

pub enum Strategy {
    Random,
    Human,
}

impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Random => write!(f, "Random"),
            Self::Human => write!(f, "Human"),
        }
    }
}


pub struct Player {
    pub strategy: Strategy,
    pub hand: Vec<Card>,
    id: u8,
    partner_id: u8,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} Player: {{ ", self.strategy).ok();
        for card in &self.hand {
            write!(f, "{}, ", card).ok();
        }
        write!(f, "}}")
    }
}

impl Player {
    pub fn new(strat: Strategy, hand: Vec<Card>, id: u8) -> Player{
        Player {
            strategy: strat,
            hand: hand,
            id: id,
            partner_id: (id + 2) % 4
        }

    }

    pub fn get_id(&self) -> &u8 {
        &self.id
    }

    pub fn get_partner_id(&self) -> &u8 {
        &self.partner_id
    }

    pub fn get_strategy(&self) -> &Strategy{
        &self.strategy
    }
}