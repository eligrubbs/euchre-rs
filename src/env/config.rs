use crate::agent::Agent;


pub struct GameConfig {
    pub agents: Vec<Box<dyn Agent>>,
    pub dealer_id: Option<u8>,
    pub seed: Option<u64>,
    pub verbose: bool,
    _private: (), // exists to prevent explicit initialization
}


impl GameConfig {
    pub fn new(agents: Vec<Box<dyn Agent>>, dealer_id: Option<u8>, seed: Option<u64>, verbose: bool) -> GameConfig{
        // Validation
        if agents.len() != 4 {
            panic!("Can only play euchre with exactly 4 players");
        }

        if dealer_id.is_some() && dealer_id.unwrap() > 3{
            panic!("Dealer ID must be between 0 and 3 inclusive");
        }

        GameConfig {
            agents: agents,
            dealer_id: dealer_id,
            seed: seed,
            verbose: verbose,
            _private: (),
        }
    }
}
