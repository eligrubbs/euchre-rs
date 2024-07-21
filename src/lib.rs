
pub mod card;
pub mod game;
pub mod player;
pub mod dealer;
pub mod utils;
pub mod judger;
pub mod env;
pub mod agent;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
