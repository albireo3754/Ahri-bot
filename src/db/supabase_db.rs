use std::env;

use axum::http::HeaderValue;
use postgrest::Postgrest;
use reqwest::Response;
use serde::Deserialize;

use super::DBManager;

pub struct SupabaseDBManager {
    client: Postgrest,    
}

impl SupabaseDBManager {
    pub fn new() -> Self {
        let api_key = env::var("SUPABASE_PUBLIC_API_KEY").unwrap();
        let client = Postgrest::new("https://veuafjcyistlfaakeecb.supabase.co/rest/v1")
            .insert_header("apiKey", api_key.clone())
            .insert_header(
                "Authorization",
                &format!("Bearer {}", api_key));
        
        SupabaseDBManager {
            client   
        }
    }

    // async fn decode<T: Deserialize<'a> + Clone>(response: &Response) -> T {
    //     let response_string = response.text().await.unwrap();
    //     let result = serde_json::from_reader::<T>(response_string.as_str());
    //     result.unwrap().clone()
    // }
}

impl DBManager for SupabaseDBManager {
    async fn select_player_with_discord_user_id(&self, discord_user_id: u64) -> Option<crate::game::Player> {
        let response = self.client
            .from("PLAYER")
            .select("*")
            // .eq("discord_id", &discord_user_id.to_string())
            .execute()
            .await
            .expect("hi");
        // let a: i32 = SupabaseDBManager::decode(response);
        println!("{}: {:?}", response.status(), response.text().await.unwrap());
        Option::None
    }

    async fn select_player_with_summoner_name(&self, summoner_name: String) -> Option<crate::game::Player> {
        todo!()
    }

    async fn select_all_player(&self) -> Vec<crate::game::Player> {
        todo!()
    }

    async fn update_player_score(&self, player_id: u64, score: i32) {
        todo!()
    }

    async fn update_players(&self, new_players: &Vec<crate::game::Player>) {
        todo!()
    }

    async fn create_player_with_discord_user_id(&self, discord_user_id: u64, summoner_name: String, tier: crate::game::Tier) -> crate::game::Player {
        todo!()
    }

    async fn create_player_with_discord_user_id_v2(&self, discord_user_id: u64) -> crate::game::Player {
        todo!()
    }

    async fn create_game(&self, game: crate::game::Game) {
        todo!()
    }

    async fn load_game(&self, game_id: i32) -> crate::game::Game {
        todo!()
    }

    async fn get_new_game_id(&self) -> i32 {
        todo!()
    }
}

mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        dotenv::dotenv().ok();
        println!("{:?}", env::var("SUPABASE_PUBLIC_API_KEY"));
        let db = SupabaseDBManager::new();
        let player = db.select_player_with_discord_user_id(1).await;
    }
}