use std::{rc::Rc, sync::Arc};

use crate::{game::{Player, Game, Tier}, db_manager::DBManger};

pub struct PlayerManager {
    db: Arc<DBManger>
}

impl PlayerManager {
    pub fn new(db: Arc<DBManger>) -> Self {
        PlayerManager { db }
    }

    pub async fn find_player_with_discord_user_id(&self, discord_user_id: u64) -> Option<Player> {
        self.db.select_player_with_discord_user_id(discord_user_id).await
    }

    pub async fn register_player(&self, discord_user_id: u64, summoner_name: String, tier: Tier) -> Player {
        if let Some(player) = self.db.select_player_with_summoner_name(summoner_name.clone()).await {
            self.db.update_discord_id_to_player(player.id, discord_user_id).await;
            return player;
        }
        self.db.create_player_with_discord_user_id(discord_user_id, summoner_name, tier).await
    }

    pub async fn save_game(&self, game: Game) {
        self.db.create_game(game).await;
    }
}