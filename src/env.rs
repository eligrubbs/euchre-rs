
use crate::game::EuchreGame;
use crate::agent::Agent;
use crate::game::scoped_state::ScopedGameState;

pub struct EuchreEnv {
    game: EuchreGame,
    agents: Vec<Box<dyn Agent>>,
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

    pub fn run(&mut self) {
        let mut state: ScopedGameState = self.game.get_state();
        let mut curr_player = state.current_actor;
        while !self.game.is_over() {
            let act = self.agents[usize::from(curr_player)].decide_action(&state);
        }
    }

}

