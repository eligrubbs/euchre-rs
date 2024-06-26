use rand::Rng;
use rand::prelude::ThreadRng;

pub mod player;
use player::{Player, Strategy};
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
            The Dealer is player {:?}, they are a {} Player.",
            dealer_ind,
            players[usize::from(dealer_ind)].strategy);

    let mut deck: Deck = Deck::new();
    deck.shuffle();
    println!("The deck is: {}", deck);

}

pub fn determine_dealer() -> u8 {
    let mut rng: ThreadRng = rand::thread_rng();
    rng.gen_range(0..=3)
}
