use std::{fs};

use itertools::Itertools;
use serde::{Deserialize, Serialize, Serializer};
use tokio::sync::Mutex;

use crate::game::{Player, Tier, Game};

use super::DBManager;

pub struct InMemoryDBManger {
    players_vec: Mutex<Vec<Player>>,
    last_game_id: Mutex<i32>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerLegacy {
    pub id: u64,
    pub discord_id: Vec<u64>,
    pub summoner_name: String,
    pub tier: Tier,
    pub score: i32,
    pub win: i32,
    pub lose: i32,
    #[serde(default = "score_deviation")]
    pub score_deviation: i32,
}

fn score_deviation() -> i32 {
    200
}

impl InMemoryDBManger {
    pub fn migrate_player(user_json_str: &str) -> Vec<Player> {
        let mut players_vec: Vec<PlayerLegacy> = serde_json::from_str(user_json_str).unwrap_or(Vec::new());

        players_vec.iter().map(|legacy| { Player::migrate(legacy.id, legacy.discord_id.first().unwrap_or(&0).clone(), legacy.score, legacy.win, legacy.lose, legacy.score_deviation)}).collect()
    }

    pub fn new() -> Self {
        let mut user_id_json_string_result = fs::read_to_string("./.data/user_id.json");
        if user_id_json_string_result.is_err() {
            fs::create_dir_all("./.data/game").unwrap();
            fs::write("./.data/user_id.json", "[]").unwrap();
            user_id_json_string_result = fs::read_to_string("./.data/user_id.json");
        }

        let user_id_json_string_result = user_id_json_string_result.unwrap();
        let user_id_json_string = user_id_json_string_result.as_str();
        let mut players_vec: Vec<Player> = serde_json::from_str(user_id_json_string).unwrap_or(Vec::new());

        // MARK: - migration example 
        // if players_vec.len() == 0 {
        //     players_vec = InMemoryDBManger::migrate_player(user_id_json_string);
        //     if players_vec.len() > 0 {
        //         fs::write("./.data/user_id.json", serde_json::to_string(&players_vec).unwrap()).unwrap();
        //         // TODO: - Game migrate logic need because game's vector has PlayerLegacy & Player
        //     }
        // }

        let mut game_count = fs::read_dir("./.data/game").unwrap().map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            file_name.replace(".json", "").parse::<i32>().unwrap()
        }).max().unwrap_or(0);
        
        InMemoryDBManger {
            players_vec: Mutex::new(players_vec),
            last_game_id: Mutex::new(game_count as i32)
        }
    }

    async fn save(&self, players_vec: Vec<Player>) {
        tokio::spawn(async move {
            println!("save {:?}", players_vec.iter().map(|player| player.score).collect::<Vec<i32>>());
            tokio::fs::write(format!("./.data/user_id.json"), serde_json::to_string(&players_vec).unwrap()).await.unwrap();
        });
    }
}

impl DBManager for InMemoryDBManger {
    async fn select_player_with_discord_user_id(&self, discord_user_id: u64) -> Option<Player> {
        let result = self.players_vec.lock().await
            .iter()
            .find(|player| player.discord_id == discord_user_id)
            .map(|player| player.clone());

        result
    }

    async fn select_player_with_summoner_name(&self, summoner_name: String) -> Option<Player> {
        let result = self.players_vec.lock().await
            .iter()
            .find(|player| player.summoner_name.to_lowercase() == summoner_name.to_lowercase())
            .map(|player| player.clone());

        result
    }

    async fn select_all_player(&self) -> Vec<Player> {
        let result = self.players_vec.lock().await;
        
        result.clone()
    }

    async fn update_player_score(&self, player_id: u64, score: i32) {
        let mut players_vec = self.players_vec.lock().await;
        let player = players_vec.iter_mut().find(|player| player.id == player_id).unwrap();
        player.score = score;

        self.save(players_vec.clone()).await;
    }

    async fn update_players(&self, new_players: &Vec<Player>) {
        let mut players_vec = self.players_vec.lock().await;
        new_players.iter().for_each(|new_player| {
            if let Some(player) = players_vec.iter_mut().find(|p| { 
                p.id == new_player.id 
            }) {    
                *player = new_player.clone();
                println!("player: {:?}, new_player: {:?} score:{:?}", player.win, new_player.win, player.score);
            }
        });

        self.save(players_vec.clone()).await;
    }

    async fn create_player(&self, player: &Player) -> bool {
        let mut players_vec = self.players_vec.lock().await;
        (*players_vec).push(player.clone());
        self.save(players_vec.clone()).await;
        true
    }

    async fn create_game(&self, game: &Game) -> bool {
        let mut last_game_id = self.last_game_id.lock().await;
        let ok = tokio::fs::write(format!("./.data/game/{}.json", game.id), serde_json::to_string(&game).unwrap()).await.is_ok();
        if ok {
            *last_game_id = game.id as i32;
        }
        ok
    }

    async fn load_game(&self, game_id: i32) -> Game {
        let raw_game = tokio::fs::read(format!("./.data/game/{}.json", game_id)).await.unwrap();
        serde_json::from_slice::<Game>(&raw_game).unwrap()
    }

    async fn get_and_increase_new_game_id(&self) -> i32 {
        let mut last_game_id = self.last_game_id.lock().await;
        *last_game_id += 1;
        return *last_game_id;
    }
}
