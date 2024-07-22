
use crate::game::EuchreGame;
use crate::agent::Agent;
use crate::game::scoped_state::ScopedGameState;

pub struct EuchreEnv {
    pub game: EuchreGame,
    pub agents: Vec<Box<dyn Agent>>,
}

impl EuchreEnv {
    pub fn new(agents: Vec<Box<dyn Agent>>) -> EuchreEnv {
        if agents.len() != 4 {
            panic!("Can only play euchre with exactly 4 players");
        }

        let game = EuchreGame::new(None);
        EuchreEnv {
            game: game,
            agents: agents,
        }
    }

    /// Run a random euchre game from start to finish.
    /// 
    /// This function returns the rewards each player got at the end of the game.
    pub fn run(&mut self) -> Vec<u8> {
        let mut state: ScopedGameState = self.game.get_state();
        let mut curr_player = state.current_actor;
        while !self.game.is_over() {
            let act: crate::utils::Action = self.agents.get(usize::from(curr_player)).unwrap().decide_action(&state);
            (state, curr_player) = self.game.step(act);
        }
        self.game.get_rewards().unwrap()
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
        let mut env: EuchreEnv = EuchreEnv::new(players);
        let rewards = env.run();
        print!("{:?}", rewards);
        assert!(4 == rewards.len())
    }
}