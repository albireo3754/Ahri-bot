mod rollback {
  use std::{collections::HashMap, sync::Arc};

  use crate::{db::{inmemory_db::InMemoryDBManger, supabase_db::SupabaseDBManager, DBManager}, game::{Player, State}, legacy::game::GameV0, player_manager::PlayerManager};

  #[tokio::test]
  #[ignore]
  async fn rollback() {
    dotenv::dotenv().unwrap();
    let db = SupabaseDBManager::new().await;
    let mut games = db.load_all_game().await;
    let mut all_player_for_rollback = db.select_all_player().await;
    if all_player_for_rollback.len() < 10 {
      panic!("No player to rollback");
    }

    // 1. player 다 뽑아내기 game
    let mut newPlayerMap: HashMap<u64, Player> = HashMap::new();
    newPlayerMap.reserve(all_player_for_rollback.len());

    for player in all_player_for_rollback.iter() {
      newPlayerMap.insert(player.id.clone(), Player::new_v2(player.id, player.discord_id));
    }

    let player_manager = PlayerManager::new(Arc::new(db));

    // println!("newPlayerMap: {:?}", newPlayerMap);
    let mut count = 0;
    for mut game in games {
      println!("{}th game: {:?}", count, game.id);
      count += 1;
      game.players_id.iter().for_each(|player_id| {
        let player = newPlayerMap.remove(player_id);

        if player.is_none() {
          println!("player_id : {} is none", player_id);
          return;
        }
        let player = player.unwrap();
        game.players.push(player);
      });
      if game.players.len() != 10 {
        game.players.iter().for_each(|player| {
          newPlayerMap.insert(player.id, player.clone());
        });
        println!("{}th game.players.len() != 10", count - 1);
        continue;
      }
      player_manager.end_game(&mut game).await;
      game.players.iter().for_each(|player| {
        newPlayerMap.insert(player.id, player.clone());
      });
    }
    // 2. games를 순회하면서 레드팀, 블루팀 승리를 처음부터 쌓아가기

    // 3. player 정보에 대한 업데이트 치기
  }
}