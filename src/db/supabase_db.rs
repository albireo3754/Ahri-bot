use std::{any::{type_name, TypeId}, backtrace::Backtrace, collections::HashMap, env};

use axum::http::HeaderValue;
use postgrest::Postgrest;
use reqwest::Response;
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{game::Player, legacy::game::GameV0};

use super::DBManager;

pub struct SupabaseDBManager {
    pub client: Mutex<Postgrest>,
    pub game_id: Mutex<i32>
}

impl SupabaseDBManager {
    pub async fn new() -> Self {
        // let api_key = env::var("SUPABASE_PUBLIC_API_KEY").unwrap();
        let api_key = env::var("SUPABASE_SERVICE_ROLL").unwrap();
        let client = Postgrest::new("https://veuafjcyistlfaakeecb.supabase.co/rest/v1")
            .insert_header("apiKey", api_key.clone())
            .insert_header(
                "Authorization",
                &format!("Bearer {}", api_key));
        
        let last_game_id = client.from("GAME")
            .select("id")
            .order("id.desc")
            .limit(1)
            .execute()
            .await;

        let last_game_id = SupabaseDBManager::handle_response(last_game_id).unwrap();
        let last_game_results = SupabaseDBManager::decode::<Vec<HashMap<String, i32>>>(last_game_id).await.unwrap_or(Vec::new());
        let last_game_id = last_game_results.first().and_then(|json| json.get("id")).unwrap_or(&0).clone();
        let supabaseDBManager = SupabaseDBManager {
            client: Mutex::new(client),
            game_id: Mutex::new(last_game_id + 1)
        };

        println!("SupabaseDB Is Launched");

        supabaseDBManager
    }

    pub async fn init_game_id(&self) {
        let mut game_id = self.game_id.lock().await;
        *game_id = self.get_and_increase_new_game_id().await;
    }

    pub fn handle_response(response: Result<Response, reqwest::Error>) -> Option<Response> {
        if let Ok(response) = response {
            if response.status().is_success() {
                return Option::Some(response);
            } else {
                println!("supabase status error: {:?}", response);
                return Option::None;
            }
        } else {
            println!("supabase setting error: {:?}", response.err());
            return Option::None;
        }
    }

    pub async fn decode<T: for<'de> Deserialize<'de>>(response: Response) -> Option<T> {
        let response_string = response.text().await.unwrap();
        let result = serde_json::from_str::<T>(response_string.as_str());
        if let Ok(result) = result {
            return Option::Some(result);
        } else {
            println!("Custom backtrace: {}", Backtrace::capture());
            println!("supabase decode error data: {}, state: {:?}", response_string, result.err());
            return Option::None;
        }
    }

    pub async fn encode<T: serde::Serialize + std::fmt::Debug>(data: T) -> Option<String> {
        let result = serde_json::to_string(&data);
        if let Ok(result) = result {
            return Option::Some(result);
        } else {
            println!("Custom backtrace: {}", Backtrace::capture());
            println!("supabase encode error data: {:?}, state: {:?}", data, result.err());
            return Option::None;
        }
    }
}

impl DBManager for SupabaseDBManager {
    async fn select_player_with_discord_user_id(&self, discord_user_id: u64) -> Option<crate::game::Player> {
        let response = self.client.lock().await
            .from("PLAYER")
            .select("*")
            .eq("discord_id", &discord_user_id.to_string())
            .single()
            .execute()
            .await;

        let response = SupabaseDBManager::handle_response(response)?;
        let player = SupabaseDBManager::decode::<crate::game::Player>(response).await;
        player
    }

    async fn select_player_with_summoner_name(&self, summoner_name: String) -> Option<crate::game::Player> {
        let response = self.client.lock().await
            .from("PLAYER")
            .select("*")
            .eq("summoner_name", &summoner_name)
            .single()
            .execute()
            .await;

        let response = SupabaseDBManager::handle_response(response)?;
        let player = SupabaseDBManager::decode::<crate::game::Player>(response).await;
        player
    }

    async fn select_all_player(&self) -> Vec<crate::game::Player> {
        let response = self.client.lock().await
            .from("PLAYER")
            .select("*")
            .execute()
            .await;

        let response = SupabaseDBManager::handle_response(response).unwrap();
        let players = SupabaseDBManager::decode::<Vec<crate::game::Player>>(response).await;
        println!("players: {:?}", players);
        players.unwrap_or(Vec::new())
    }

    async fn update_player_score(&self, player_id: u64, score: i32) {
        let response = self.client.lock().await
            .from("PLAYER")
            .eq("id", &player_id.to_string())
            .update("{ \"score\": \"$score\" }")
            .execute()
            .await;

        SupabaseDBManager::handle_response(response);
    }

    async fn update_players(&self, new_players: &Vec<crate::game::Player>) {
        let updated_players = SupabaseDBManager::encode(new_players).await;
        if updated_players.is_none() {
            return;
        }
        let updated_players = updated_players.unwrap();
        let response = self.client.lock().await
            .from("PLAYER")
            .upsert(updated_players)
            .execute()
            .await;

        SupabaseDBManager::handle_response(response);
    }

    async fn create_player(&self, player: &crate::game::Player) -> bool {
        let encoded_player = SupabaseDBManager::encode(player.clone()).await.unwrap();
        let response = self.client.lock().await
            .from("PLAYER")
            .insert(encoded_player)
            .execute()
            .await;

        SupabaseDBManager::handle_response(response).is_some()
    }

    async fn create_game(&self, game: &crate::game::Game) -> bool {
        let encoded_game = SupabaseDBManager::encode(game.clone()).await.unwrap();
        let response = self.client.lock().await
            .from("GAME")
            .insert(encoded_game)
            .execute()
            .await;

        SupabaseDBManager::handle_response(response).is_some()
    }

    async fn load_game(&self, game_id: i32) -> crate::game::Game {
        let response = self.client.lock().await
            .from("GAME")
            .select("*")
            .eq("id", &game_id.to_string())
            .single()
            .execute()
            .await;

        let response = SupabaseDBManager::handle_response(response).unwrap();
        let game = SupabaseDBManager::decode::<crate::game::Game>(response).await;
        game.unwrap()
    }

    async fn load_all_game(&self) -> Vec::<crate::game::Game> {
        let response = self.client.lock().await
            .from("GAME")
            .select("*")
            .order("id")
            .execute()
            .await;

        let response = SupabaseDBManager::handle_response(response).unwrap();
        let game = SupabaseDBManager::decode::<Vec::<crate::game::Game>>(response).await;
        game.unwrap()
    }
    async fn get_and_increase_new_game_id(&self) -> i32 {
        let mut game_id = self.game_id.lock().await;
        let new_game_id = *game_id;
        *game_id += 1;
        return new_game_id;
    }
}

mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn supabase_init_test() {
        dotenv::dotenv().ok();
        let _ = SupabaseDBManager::new();
    }
}