use super::player::Player;

pub struct Game {
    pub players: Vec<Player>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: Vec::new(),
        }
    }

    pub fn create_player(&mut self, name: &str, character_id: &str) -> Player {
        let player = Player::new(name, character_id);
        self.add_player(player.clone());
        player
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn remove_player(&mut self, player: Player) {
        self.players.retain(|p| p.get_name() != player.get_name());
    }

    pub fn get_player(&self, name: &str) -> Option<&Player> {
        self.players.iter().find(|p| p.get_name() == name)
    }

    pub fn get_player_mut(&mut self, name: &str) -> Option<&mut Player> {
        self.players.iter_mut().find(|p| p.get_name() == name)
    }

    pub fn get_players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn get_players_mut(&mut self) -> &mut Vec<Player> {
        &mut self.players
    }

    pub fn get_player_names(&self) -> Vec<String> {
        self.players
            .iter()
            .map(|p| p.get_name().to_string())
            .collect()
    }

    pub fn get_player_character_ids(&self) -> Vec<String> {
        self.players
            .iter()
            .map(|p| p.get_character_id().to_string())
            .collect()
    }
}
