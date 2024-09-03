use std::io::stdin;
use euchre_rs::env::{EuchreEnv, config::GameConfig};
use euchre_rs::agent::{Agent, human::HumanAgent, random::RandomAgent};

/// Play a euchre game as a human
fn main() {
    let agents:Vec<Box<dyn Agent>> = vec![Box::new(HumanAgent::new(stdin().lock())),
                                          Box::new(RandomAgent{}),
                                          Box::new(RandomAgent{}),
                                          Box::new(RandomAgent{})];

    let config: GameConfig = GameConfig::new(agents, Some(0), Some(12));
    let mut env: EuchreEnv = EuchreEnv::new(config);
    env.run();
}   
