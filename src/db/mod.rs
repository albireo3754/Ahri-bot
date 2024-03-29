use crate::game::{Player, Tier, Game};
pub mod inmemory_db;
pub mod supabase_db;

pub trait DBManager {
    async fn select_player_with_discord_user_id(&self, discord_user_id: u64) -> Option<Player>;
    async fn select_player_with_summoner_name(&self, summoner_name: String) -> Option<Player>;
    async fn select_all_player(&self) -> Vec<Player>;
    async fn update_player_score(&self, player_id: u64, score: i32);
    async fn update_players(&self, new_players: &Vec<Player>);
    async fn create_player(&self, player: &Player) -> bool;
    async fn create_game(&self, game: &Game) -> bool;
    async fn load_game(&self, game_id: i32) -> Game;
    async fn get_and_increase_new_game_id(&self) -> i32;
}