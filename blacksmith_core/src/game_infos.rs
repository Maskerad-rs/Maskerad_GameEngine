

#[derive(Debug)]
pub struct GameInfos {
    author: String,
    game_name: String,
}

impl GameInfos {
    pub fn new(game_name: &str, author: &str) -> Self {
        GameInfos {
            author: author.to_string(),
            game_name: game_name.to_string(),
        }
    }

    pub fn author(&self) -> String {
        self.author.clone()
    }

    pub fn game_name(&self) -> String {
        self.game_name.clone()
    }
}