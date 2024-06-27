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