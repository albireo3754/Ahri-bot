

// ==================================
//
// bin으로 migration로직을 관리하려면 lib으로 따로 빼야하는데 일이 많으므로 테스트코드에서 해결한다.
//
// ===================================

mod migration_v1 {
  use std::fs;
  use crate::{db::{inmemory_db::InMemoryDBManger, supabase_db::SupabaseDBManager, DBManager}, game::{Player, State}, legacy::game::GameV0};
  use serde::{Deserialize, Serialize};

  #[tokio::test]
  #[ignore]
  async fn migration_inmemory_db_game_v0_to_supabse_db() {
      dotenv::dotenv().unwrap();
      let mut user_id_json_string_result = fs::read_to_string("./.data/user_id.json");
      if user_id_json_string_result.is_err() {
          fs::create_dir_all("./.data/game").unwrap();
          fs::write("./.data/user_id.json", "[]").unwrap();
          user_id_json_string_result = fs::read_to_string("./.data/user_id.json");
      }
  
      let user_id_json_string_result = user_id_json_string_result.unwrap();
      let user_id_json_string = user_id_json_string_result.as_str();
      let mut players_vec: Vec<Player> = serde_json::from_str(user_id_json_string).unwrap_or(Vec::new());
  
  
      let mut all_game = fs::read_dir("./.data/game").unwrap().map(|entry| {
          let entry = entry.unwrap();
          let path = entry.path();
          let file_name = path.file_name().unwrap().to_str().unwrap();
          file_name.replace(".json", "").parse::<i32>().unwrap()
      })
      .map(|game_id| {
          let raw_game = fs::read(format!("./.data/game/{}.json", game_id)).unwrap();
          serde_json::from_slice::<GameV0>(&raw_game).unwrap()
      });
  
      let supabase_db = SupabaseDBManager::new().await;
  
      for game in all_game {
          supabase_db.create_game(game).await;
      }
  }

  
  // MARK: - player_v0은 이미 다른 로직으로 player_v1으로 마이그레이션을 마친상태
  #[tokio::test]
  #[ignore]
  async fn migration_inmemory_db_player_v1_to_supabse_db() {
    dotenv::dotenv().unwrap();
    let inmemory_db = InMemoryDBManger::new();
    let supabase_db = SupabaseDBManager::new().await;

    let all_players = inmemory_db.select_all_player().await;
    for player in all_players {
      println!("player: {:?}", player);
      assert!(supabase_db.create_player(&player).await);
    }
  }

  impl SupabaseDBManager {
    pub async fn create_game(&self, game: GameV0) -> bool {
        let encoded_game = SupabaseDBManager::encode(game.clone()).await.unwrap();
        let response = self.client.lock().await
            .from("GAME")
            .insert(encoded_game)
            .execute()
            .await;

        SupabaseDBManager::handle_response(response).is_some()
    }
  }
}



