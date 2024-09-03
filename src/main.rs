use euchre_rs::env::{EuchreEnv, config::GameConfig};
use euchre_rs::agent::{Agent, human::HumanAgent, random::RandomAgent};

/// Play a euchre game as a human
fn main() {
    let agents:Vec<Box<dyn Agent>> = vec![Box::new(HumanAgent::default()),
                                          Box::new(RandomAgent{}),
                                          Box::new(RandomAgent{}),
                                          Box::new(RandomAgent{})];

    let config: GameConfig = GameConfig::new(agents, Some(1), Some(12), true);
    let mut env: EuchreEnv = EuchreEnv::new(config);
    let results: Vec<u8> = env.run();
    println!("Game score: {:?}", results);
}   
