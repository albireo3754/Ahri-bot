mod rollback {
  use std::collections::HashMap;

  use crate::{db::{inmemory_db::InMemoryDBManger, supabase_db::SupabaseDBManager, DBManager}, game::{Player, State}, legacy::game::GameV0};

  #[tokio::test]
  #[ignore]
  async fn rollback() {
    dotenv::dotenv().unwrap();
    let db = SupabaseDBManager::new().await;
    let games = db.load_all_game().await;

    // 1. player 다 뽑아내기 game
    let mut playerMap: HashMap<i64, Player> = HashMap::new();

    // 2. games를 순회하면서 레드팀, 블루팀 승리를 처음부터 쌓아가기

    // 3. player 정보에 대한 업데이트 치기
  }
}