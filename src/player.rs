

pub struct Player {
    name: String,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_player() {
        let play: Player = Player::new("bob");
        let real_name: String = "bob".to_string();
        assert_eq!(play.name, real_name)
    }
}