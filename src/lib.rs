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

    let mut deck: Deck = Deck::new();
    deck.shuffle();

    let players: Vec<Player> = vec![
        Player::new(Strategy::Human,  deck.deal_n_cards(5), 0),
        Player::new(Strategy::Random,  deck.deal_n_cards(5), 1),
        Player::new(Strategy::Random,  deck.deal_n_cards(5), 2),
        Player::new(Strategy::Random,  deck.deal_n_cards(5), 3),
    ];

    let dealer_ind: u8 = determine_dealer();

    let p1_strat: &Strategy = & players[usize::from(dealer_ind)].strategy;

    println!("Here is the lineup: \n\n\
            You are player 0 \n\
            The Dealer is player {:?}, they are a {} Player.\n",
            dealer_ind,
            p1_strat);


    println!("Your hand is: {}", players[0]);

}

pub fn determine_dealer() -> u8 {
    let mut rng: ThreadRng = rand::thread_rng();
    rng.gen_range(0..=3)
}
