use crate::game::scoped_state::ScopedGameState;
use crate::action::Action;

pub mod random;
pub mod human;

pub trait Agent{
    fn decide_action(&mut self, state: &ScopedGameState) -> Action;
}
