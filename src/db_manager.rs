use std::{collections::HashMap, fs, path};

use tokio::{fs::File, io::AsyncReadExt, sync::Mutex};

use crate::game::{Player, Game, Tier};

pub struct DBManger {
    players_vec: Mutex<Vec<Player>>,
    last_game_id: Mutex<i32>
}

impl DBManger {
    pub fn new() -> Self {
        let mut user_id_json_string_result = fs::read_to_string("./.data/user_id.json");
        if user_id_json_string_result.is_err() {
            fs::create_dir_all("./.data/game").unwrap();
            fs::write("./.data/user_id.json", "[]").unwrap();
            user_id_json_string_result = fs::read_to_string("./.data/user_id.json");
        }
        let players_vec: Vec<Player> = serde_json::from_str(user_id_json_string_result.unwrap().as_str()).unwrap_or(Vec::new());
        let mut game_count = fs::read_dir("./.data/game").unwrap().map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            file_name.replace(".json", "").parse::<i32>().unwrap()
        }).max().unwrap_or(0);
        DBManger {
            players_vec: Mutex::new(players_vec),
            last_game_id: Mutex::new(game_count as i32)
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

    pub async fn select_all_player(&self) -> Vec<Player> {
        let result = self.players_vec.lock().await;
        
        result.clone()
    }

    pub async fn update_discord_id_to_player(&self, player_id: u64, discord_user_id: u64) {
        let mut players_vec = self.players_vec.lock().await;
        let player = players_vec.iter_mut().find(|player| player.id == player_id).unwrap();
        player.discord_id.push(discord_user_id);

        self.save(players_vec.clone()).await;
    }

    pub async fn update_player_score(&self, player_id: u64, score: i32) {
        let mut players_vec = self.players_vec.lock().await;
        let player = players_vec.iter_mut().find(|player| player.id == player_id).unwrap();
        player.score = score;

        self.save(players_vec.clone()).await;
    }

    pub async fn update_players(&self, new_players: &Vec<Player>) {
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

    pub async fn create_player_with_discord_user_id(&self, discord_user_id: u64, summoner_name: String, tier: Tier) -> Player {
        let mut players_vec = self.players_vec.lock().await;
        let player = Player::new((players_vec.len() + 1) as u64, discord_user_id, summoner_name, tier);
        (*players_vec).push(player.clone());
        
        self.save(players_vec.clone()).await;
        player
    }

    pub async fn create_player_with_discord_user_id_v2(&self, discord_user_id: u64) -> Player {
        let mut players_vec = self.players_vec.lock().await;
        let player = Player::new_v2((players_vec.len() + 1) as u64, discord_user_id);
        (*players_vec).push(player.clone());
        
        self.save(players_vec.clone()).await;
        player
    }
    async fn save(&self, players_vec: Vec<Player>) {

        tokio::spawn(async move {
            println!("save {:?}", players_vec.iter().map(|player| player.score).collect::<Vec<i32>>());
            tokio::fs::write(format!("./.data/user_id.json"), serde_json::to_string(&players_vec).unwrap()).await.unwrap();
        });
    }

    pub async fn create_game(&self, game: Game) {
        let mut last_game_id = self.last_game_id.lock().await;
        tokio::fs::write(format!("./.data/game/{}.json", game.id), serde_json::to_string(&game).unwrap()).await.unwrap();
        *last_game_id = game.id as i32;
    }

    pub async fn load_game(&self, game_id: i32) -> Game {
        let raw_game = tokio::fs::read(format!("./.data/game/{}.json", game_id)).await.unwrap();
        serde_json::from_slice::<Game>(&raw_game).unwrap()
    }

    pub async fn get_new_game_id(&self) -> i32 {
        let mut last_game_id = self.last_game_id.lock().await;
        *last_game_id += 1;
        return *last_game_id;
    }
}