use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub name: String,
    pub character_id: String,
}

impl Player {
    pub fn new(name: &str, character_id: &str) -> Player {
        Player {
            name: name.to_string(),
            character_id: character_id.to_string(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_character_id(&self) -> &str {
        &self.character_id
    }
}
