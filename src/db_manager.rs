use std::{collections::HashMap, fs, path};

use tokio::{fs::File, io::AsyncReadExt, sync::Mutex};

use crate::game::{Player, Game, Tier};

pub struct DBManger {
    players_vec: Mutex<Vec<Player>>,
    save_queue: Mutex<i32>
}

impl DBManger {
    pub fn new() -> Self {
        let mut user_id_json_string_result = fs::read_to_string("data/user_id.json");
        if user_id_json_string_result.is_err() {
            fs::create_dir("./data").unwrap();
            fs::write("./data/user_id.json", "[]").unwrap();
            user_id_json_string_result = fs::read_to_string("data/user_id.json");
        }
        let players_vec: Vec<Player> = serde_json::from_str(user_id_json_string_result.unwrap().as_str()).unwrap_or(Vec::new());

        DBManger {
            players_vec: Mutex::new(players_vec),
            save_queue: Mutex::new(0)
        }
    }

    pub async fn select_player_with_discord_user_id(&self, discord_user_id: u64) -> Option<Player> {
        let result = self.players_vec.lock().await
            .iter()
            .find(|player| player.discord_id.contains(&discord_user_id))
            .map(|player| player.clone());

        result
    }

    pub async fn select_player_with_summoner_name(&self, summoner_name: String) -> Option<Player> {
        let result = self.players_vec.lock().await
            .iter()
            .find(|player| player.summoner_name.to_lowercase() == summoner_name.to_lowercase())
            .map(|player| player.clone());

        result
    }

    pub async fn update_discord_id_to_player(&self, player_id: u64, discord_user_id: u64) {
        let mut players_vec = self.players_vec.lock().await;
        let player = players_vec.iter_mut().find(|player| player.id == player_id).unwrap();
        player.discord_id.push(discord_user_id);

        self.save(players_vec.clone()).await;
    }

    pub async fn create_player_with_discord_user_id(&self, discord_user_id: u64, summoner_name: String, tier: Tier) -> Player {
        let mut players_vec = self.players_vec.lock().await;
        let player = Player::new((players_vec.len() + 1) as u64, discord_user_id, summoner_name, tier);
        (*players_vec).push(player.clone());
        
        self.save(players_vec.clone()).await;
        player
    }

    async fn save(&self, players_vec: Vec<Player>) {
        tokio::spawn(async move {
            tokio::fs::write(format!("data/user_id.json"), serde_json::to_string(&players_vec).unwrap()).await.unwrap();
        });
    }

    pub async fn create_game(&self, game: Game) {
        tokio::spawn(async move {
            tokio::fs::write(format!("data/game/{}.json", game.id), serde_json::to_string(&game).unwrap()).await.unwrap();
        });
    }
}