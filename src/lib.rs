use std::fmt;
use rand::Rng;
use rand::prelude::ThreadRng;

pub mod card;
pub mod deck;
use deck::Deck;

/// Function to run Euchre.
pub fn run() {
    println!("This is Euchre!");

    let players: Vec<Player> = vec![
        Player {strategy: Strategy::Human},
        Player {strategy: Strategy::Random},
        Player {strategy: Strategy::Random},
        Player {strategy: Strategy::Random}
    ];

    let dealer_ind: u8 = determine_dealer();

    println!("Here is the lineup: \n\n\
            You are player 0 \n\
            The Dealer is player {:?}", dealer_ind);

    let deck: Deck = Deck::new();
    println!("The deck is: {}", deck);

}

pub fn determine_dealer() -> u8 {
    let mut rng: ThreadRng = rand::thread_rng();
    rng.gen_range(0..=3)
}

#[derive(Debug)]
enum Strategy {
    Random,
    Human,
}

impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct Player {
    strategy: Strategy,
}