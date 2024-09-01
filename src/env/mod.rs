
use crate::game::EuchreGame;
use crate::agent::Agent;
use crate::game::scoped_state::ScopedGameState;

pub mod config;
use self::config::GameConfig;

pub struct EuchreEnv {
    pub game: EuchreGame,
    pub agents: Vec<Box<dyn Agent>>,
}

impl EuchreEnv {
    pub fn new(config: GameConfig) -> EuchreEnv {

        let game = EuchreGame::new(config.dealer_id, config.seed);
        EuchreEnv {
            game: game,
            agents: config.agents,
        }
    }

    /// Run a random euchre game from start to finish.
    /// 
    /// This function returns the rewards each player got at the end of the game.
    pub fn run(&mut self) -> Vec<u8> {
        self.reset();

        let mut state: ScopedGameState = self.game.get_state();
        let mut curr_player = state.current_actor;
        while !self.game.is_over() {
            let act: crate::utils::Action = self.agents.get_mut(usize::from(curr_player)).unwrap().decide_action(&state);
            (state, curr_player) = self.game.step(act);
        }
        self.game.get_rewards().unwrap()
    }

    /// Create a new EuchreGame
    /// Game is created with random seed.
    pub fn reset(&mut self) {
        self.game = EuchreGame::new(None, None);
    }

}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::random::RandomAgent;

    #[test]
    fn run_game() {
        let players: Vec<Box<dyn Agent>> = vec![Box::new(RandomAgent{}),
                                                Box::new(RandomAgent{}),
                                                Box::new(RandomAgent{}),
                                                Box::new(RandomAgent{})];
        let config: GameConfig = GameConfig::new(players, None, None);
        let mut env: EuchreEnv = EuchreEnv::new(config);
        let rewards: Vec<u8> = env.run();
        print!("{:?}", rewards);
        assert!(4 == rewards.len())
    }
}