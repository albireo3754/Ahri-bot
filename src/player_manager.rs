use std::{rc::Rc, sync::Arc};

use rand::seq::SliceRandom;

use crate::{game::{Player, Game, Tier}, db_manager::DBManger};

pub struct PlayerManager {
    db: Arc<DBManger>
}

impl PlayerManager {
    pub fn new(db: Arc<DBManger>) -> Self {
        PlayerManager { db }
    }

    pub async fn end_game(&self, game: Game) {
        let mut game = game;
        // // sum all players win/lose
        game.players.iter().any(|player| { player.win + player.lose < 3 });
        let scores = [150, 130, 100, 130, 150];
        
        
        let mut win_player = match game.is_red_winner() {
            true => game.mut_red_players(),
            false => game.mut_blue_players()
        };
        {   
            let mut rng = rand::thread_rng();
            (*win_player).shuffle(&mut rng);
        }

        for i in 0..5 {
            win_player[i].win(scores[i]);
        }

        let mut lose_player = match game.is_red_winner() {
            false => game.mut_red_players(),
            true => game.mut_blue_players()
        };
        {
            let mut rng = rand::thread_rng();
            (*lose_player).shuffle(&mut rng);
        }

        for i in 0..5 {
            lose_player[i].lose(scores[i]);
        }
        // print players score
        println!("{:?}", game.players.iter().map(|player| player.score).collect::<Vec<i32>>());
        self.db.update_players(&game.players).await;
        self.db.create_game(game).await;
    }

    fn hande_win_player(&self, player: &mut Player, score: i32) {
        player.win += 1;
        player.score += score;
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

    pub async fn generate_game(&self, host: Player) -> Game {

        return Game::new(self.db.get_new_game_id().await as u64, host);
    }
}