use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;

use crate::game::scoped_state::ScopedGameState;
use crate::action::Action;
use crate::agent::Agent;
use crate::utils::get_rdm_gen;

pub struct RandomAgent {
    gen: ChaCha8Rng,
    _private: (), // prevents explicit instantiation
}

impl Agent for RandomAgent {
    fn decide_action(&mut self, state: &ScopedGameState) -> Action {
        let actions: &Vec<Action> = &state.legal_actions;

        actions.choose(&mut self.gen).unwrap().clone()
    }
}

impl RandomAgent {
    pub fn new(seed: Option<u64>) -> Self {
        Self {
            gen: get_rdm_gen(seed),
            _private: (),
        }
    }
}
