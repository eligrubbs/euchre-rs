use std::io::{stdin,stdout,Write};
use std::str::FromStr;

use crate::game::scoped_state::ScopedGameState;
use crate::utils::Action;
use crate::agent::Agent;

pub struct HumanAgent {

}

impl Agent for HumanAgent {
    fn decide_action(&self, state: &ScopedGameState) -> Action {
        let options: &Vec<Action> = &state.legal_actions;
        println!("{:?}", options);

        let action: Action;
        loop {
            let input = Self::get_input();
            match Action::from_str(&input) {
                Ok(act) => { action = act; break},
                Err(_e) => { },
            }
        }
        action
    }
}

impl HumanAgent {

    /// Accept terminal input from user.
    pub fn get_input() -> String {
        let mut s: String=String::new();
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use crate::{agent::random::RandomAgent, env::EuchreEnv};

    use super::*;

    #[test]
    fn get_human_action() {
        let players: Vec<Box<dyn Agent>> = vec![Box::new(HumanAgent{}), Box::new(RandomAgent{}), Box::new(RandomAgent{}), Box::new(RandomAgent{})];
        let env: EuchreEnv = EuchreEnv::new(players);
        let start: ScopedGameState = env.game.get_state();
        let act = env.agents[0].decide_action(&start);

        assert!(start.legal_actions.contains(&act));

    }
}
