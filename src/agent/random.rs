use rand::seq::SliceRandom;

use crate::game::scoped_state::ScopedGameState;
use crate::utils::Action;
use crate::agent::Agent;

pub struct RandomAgent {

}

impl Agent for RandomAgent {
    fn decide_action(&mut self, state: &ScopedGameState) -> Action {
        let actions: &Vec<Action> = &state.legal_actions;

        actions.choose(&mut rand::thread_rng()).unwrap().clone()
    }
}
