
use crate::game::EuchreGame;
use crate::agent::Agent;
use crate::game::scoped_state::ScopedGameState;
use crate::utils::Action;

pub mod config;
use self::config::GameConfig;

pub struct EuchreEnv {
    pub game: EuchreGame,
    pub config: GameConfig,

    action_history: Vec<(u8, Action)>,
}

impl EuchreEnv {
    pub fn new(config: GameConfig) -> EuchreEnv {

        let game: EuchreGame = EuchreGame::new(config.dealer_id,
                                               config.seed,
                                               );
        EuchreEnv {
            game: game,
            config: config,
            action_history: vec![],
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
            self.log_important_game_info(&state);
            let act: crate::utils::Action = self.config.agents.get_mut(usize::from(curr_player)).unwrap().decide_action(&state);
            self.record_action((curr_player, act));
            (state, curr_player) = self.game.step(act);
        }
        self.game.get_rewards().unwrap()
    }

    /// Create a new EuchreGame re-using config information.  
    pub fn reset(&mut self) {
        self.game = EuchreGame::new(self.config.dealer_id, self.config.seed);
    }

    /// Returns a read only reference for the agent with index of `index`.  
    /// Result will contain `None` if invalid index is passed.
    pub fn get_agent(&self, index: usize) -> Option<&Box<dyn Agent>> {
        self.config.agents.get(index)
    }

    /// Returns a mutable only reference for the agent with index of `index`.  
    /// Result will contain `None` if invalid index is passed.
    pub fn get_mut_agent(&mut self, index: usize) -> Option<&mut Box<dyn Agent>> {
        self.config.agents.get_mut(index)
    }

    /// Returns the id of the player who took this action, given the game state.
    fn record_action(&mut self, record: (u8, Action)) {
        self.action_history.push(record);
        self.log_action_took(&record);
    }

    /// Log the action that was taken after successfully advancing the game. 
    fn log_action_took(&self, record: &(u8, Action)) {
        // TODO: log that it happened in some type of logger.

        if self.config.verbose {
            if (record.1 as i32) < 30 || 
               (self.action_history.last().is_some() && // should always be true if we're here. but you never know...
               self.action_history.last().unwrap().0 == record.0) {
                // don't print what was discarded unless you did it
                println!("Player {}: {}", record.0, record.1);
            }
            
        }
    }

    /// Given the current state, log important information that make the player's actions make sense.
    fn log_important_game_info(&self, state: &ScopedGameState) {
        if self.action_history.len() == 0 {
            if self.config.verbose {
                println!("Flipped: {}", state.flipped_card)
            }
        }
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
        let config: GameConfig = GameConfig::new(players, None, None, false);
        let mut env: EuchreEnv = EuchreEnv::new(config);
        let rewards: Vec<u8> = env.run();
        print!("{:?}", rewards);
        assert!(4 == rewards.len())
    }
}