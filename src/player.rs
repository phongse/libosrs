use serde::Serialize;

use crate::{Gamemode, Hiscore};

#[derive(Debug, Serialize)]
pub struct Player {
    pub username: String,
    pub gamemode: Gamemode,
    pub hiscore: Hiscore,
}

impl Player {
    pub fn build_player(name: &str, hiscore: Hiscore, gamemode: Gamemode) -> Player {
        Player {
            username: name.to_string(),
            gamemode,
            hiscore,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
