use std::fmt;

#[derive(Debug)]
pub enum Strategy {
    Random,
    Human,
}

impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Player {
    pub strategy: Strategy,
}