use euchre_rs::env::{EuchreEnv, config::GameConfig};
use euchre_rs::agent::{Agent, human::HumanAgent, random::RandomAgent};

/// Play a euchre game as a human
fn main() {
    // play_one_game();
    time_to_play_x_games(1_000_000);
}   

fn play_one_game() {
    let agents:Vec<Box<dyn Agent>> = vec![Box::new(HumanAgent::default()),
    Box::new(RandomAgent::new(Some(42))),
    Box::new(RandomAgent::new(Some(42))),
    Box::new(RandomAgent::new(Some(42)))];

    let config: GameConfig = GameConfig::new(agents, Some(1), Some(12), true);
    let mut env: EuchreEnv = EuchreEnv::new(config);
    let results: Vec<u8> = env.run();
    println!("Game score: {:?}", results);
}

/// Time to play `x` games.
/// 
/// If the time exceeds 3 minutes, the function returns false.
fn time_to_play_x_games(x: u64) -> bool {
    use std::time::Instant;
    let max_runtime: u64 = 60 * 3;
    let start: Instant = Instant::now();

    // Setup
    let agents:Vec<Box<dyn Agent>> = vec![Box::new(RandomAgent::new(None)),
                                          Box::new(RandomAgent::new(None)),
                                          Box::new(RandomAgent::new(None)),
                                          Box::new(RandomAgent::new(None)),];

    let config: GameConfig = GameConfig::new(agents, None, None, false);
    let mut env: EuchreEnv = EuchreEnv::new(config);

    // Run
    let mut finished = 0;
    while finished < x {
        if start.elapsed().as_secs() >= max_runtime {
            return false
        }

        env.run();
        env.reset();

        finished += 1;
    }

    println!("Completed {} games in {} seconds.", x, start.elapsed().as_secs_f64());
    true
}