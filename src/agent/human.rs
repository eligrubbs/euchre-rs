use std::io::{BufRead,stdout, Write};
use std::str::FromStr;

use crate::game::scoped_state::ScopedGameState;
use crate::utils::Action;
use crate::agent::Agent;

pub struct HumanAgent<R>
where R: BufRead {
    reader: R
}

impl<R: BufRead> Agent for HumanAgent<R> {
    fn decide_action(&mut self, state: &ScopedGameState) -> Action {
        let options: &Vec<Action> = &state.legal_actions;
        println!("{:?}", options);

        let mut action: Action;
        loop {
            let input: String = self.get_input();
            match Action::from_str(&input) {
                Ok(act) => { 
                    action = act;
                    if options.contains(&action){
                        break
                    } else {
                        println!("Action {} not available", action);
                    }
                },
                Err(_e) => { },
            }
        }
        action
    }
}

impl<R> HumanAgent<R>
where R: BufRead {

    pub fn new(reader: R) -> Self
    {
        Self{
            reader: reader
        }
    }

    /// Accept terminal input from user.
    pub fn get_input(&mut self) -> String
    {
        let mut s: String=String::new();
        let _ = stdout().flush();
        self.reader.read_line(&mut s).expect("Did not enter a correct string");
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
    use std::io::Cursor;

    use crate::{agent::random::RandomAgent, env::{EuchreEnv, config::GameConfig},};

    use super::*;

    #[test]
    fn get_human_action() {
        let act_cursor:Cursor<&str> = Cursor::new("pICk\n");
        let players: Vec<Box<dyn Agent>> = vec![Box::new(HumanAgent::new(act_cursor)),
                                                Box::new(RandomAgent{}),
                                                Box::new(RandomAgent{}),
                                                Box::new(RandomAgent{})];
        let config: GameConfig = GameConfig::new(players, None, None);
        let mut env: EuchreEnv = EuchreEnv::new(config);
        let start: ScopedGameState = env.game.get_state();
        let act: Action = env.agents[0].decide_action(&start);

        assert_eq!(act, Action::Pick);

    }
}
