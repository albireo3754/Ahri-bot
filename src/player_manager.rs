use std::{rc::Rc, sync::Arc};

use glicko2::{GlickoRating, GameResult};
use rand::seq::SliceRandom;

use crate::{game::{Player, Game, Tier}, db_manager::{DBManager, InMemoryDBManger}};


pub struct PlayerManager<DB> where DB: DBManager {
    db: Arc<DB>
}

impl<DB> PlayerManager<DB> where DB: DBManager {
    pub fn new(db: Arc<DB>) -> Self {
        PlayerManager { db }
    }
    
    pub async fn end_game(&self, mut game: &mut Game) {
        let win_player = match game.is_red_winner() {
            true => game.red_players(),
            false => game.blue_players()
        };

        let lose_player = match game.is_red_winner() {
            false => game.red_players(),
            true => game.blue_players()
        };

        let mut new_win_score = Vec::with_capacity(5);
        let mut new_lose_score = Vec::with_capacity(5);

        let mut results = vec![];
        for j in 0..5 {
            results.push(GameResult::win(GlickoRating {
                value: f64::from(lose_player[j].score),
                deviation: f64::from(lose_player[j].score_deviation),
            }));
        }

        for i in 0..5 {
            let before_rating = GlickoRating { 
                value: f64::from(win_player[i].score), 
                deviation: f64::from(win_player[i].score_deviation) 
            };

            let new_rating: GlickoRating = glicko2::new_rating(before_rating.into(), &results, 0.5).into();
            new_win_score.push(new_rating);
        }

        let mut results = vec![];
        for j in 0..5 {
            results.push(GameResult::loss(GlickoRating {
                value: f64::from(win_player[j].score),
                deviation: f64::from(win_player[j].score_deviation),
            }));
        }

        for i in 0..5 {
            let before_rating = GlickoRating { 
                value: f64::from(lose_player[i].score), 
                deviation: f64::from(lose_player[i].score_deviation) 
            };

            let new_rating: GlickoRating = glicko2::new_rating(before_rating.into(), &results, 0.5).into();
            new_lose_score.push(new_rating);
        }

        let mut win_player = match game.is_red_winner() {
            true => game.mut_red_players(),
            false => game.mut_blue_players()
        };

        win_player.iter_mut().zip(new_win_score.iter()).for_each(|(player, score)| {
            player.win(score.value as i32 + 30, score.deviation as i32 + 15);
        });

        let mut lose_player = match game.is_red_winner() {
            false => game.mut_red_players(),
            true => game.mut_blue_players()
        };

        lose_player.iter_mut().zip(new_lose_score.iter()).for_each(|(player, score)| {
            player.lose(score.value as i32, score.deviation as i32 + 15);
        });

        // print players score
        println!("{:?}", game.players.iter().map(|player| player.score).collect::<Vec<i32>>());
        self.db.update_players(&game.players).await;
        self.db.create_game(game.clone()).await;
    }

    pub async fn find_all_player(&self) -> Vec<Player> {
        self.db.select_all_player().await
    }

    pub async fn find_player_with_discord_user_id(&self, discord_user_id: u64) -> Option<Player> {
        self.db.select_player_with_discord_user_id(discord_user_id).await
    }

    pub async fn register_player(&self, discord_user_id: u64, summoner_name: String, tier: Tier) -> Player {
        if let Some(player) = self.db.select_player_with_summoner_name(summoner_name.clone()).await {
            return player;
        }
        self.db.create_player_with_discord_user_id(discord_user_id, summoner_name, tier).await
    }

    pub async fn register_player_v2(&self, discord_user_id: u64) -> Player {
        self.db.create_player_with_discord_user_id_v2(discord_user_id).await
    }

    pub async fn generate_game(&self, host: Player) -> Game {

        return Game::new(self.db.get_new_game_id().await as u64, host);
    }
}