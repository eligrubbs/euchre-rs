
use crate::game::scoped_state::ScopedGameState;
use crate::utils::Action;

pub mod random;
pub mod human;

pub trait Agent {
    fn decide_action(&self, state: &ScopedGameState) -> Action;
}
