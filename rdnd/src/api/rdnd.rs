use super::game::Game;
use uuid::Uuid;

pub struct Rdnd {
    game: Option<Game>,
    uuid: String,
}

impl Rdnd {
    pub fn new(game_id: Option<String>) -> Rdnd {
        Rdnd {
            game: Some(Game::new()),
            uuid: game_id.unwrap_or(Rdnd::generate_game_id()),
        }
    }

    fn generate_game_id() -> String {
        Uuid::new_v4().to_string()
    }
}
