use std::{cmp::Ordering, ops::{Index, IndexMut}, borrow::BorrowMut};

use itertools::Itertools;
use rand::{Rng, seq::SliceRandom, random};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum State {
    queue, ready, result(bool), board
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub id: u64,
    pub players: Vec<Player>,
    pub state: State,
    pub team_bit: i32
}


impl Game {
    pub fn new(id: u64, host: Player) -> Game {
        let mut players = vec![host];
        players.reserve(10);
        Game {
            id: id,
            players: players,
            state: State::queue,
            team_bit: 0b11111
        }
    }

    pub fn is_red_winner(&self) -> bool {
        match self.state {
            State::result(is_blue_win) => !is_blue_win,
            _ => false
        }
    }

    pub fn add_player(&mut self, player: Player) -> bool {
        // check the player is already in self.players
        if self.players.iter().any(|p| p.id == player.id) {
            return false
        }

        self.players.push(player);
        if self.players.len() == 10 {
            self.state = State::ready;
        }
        return true
    }

    pub fn remove_player(&mut self, player_id: u64) -> bool {
        let before_player_lens = self.players.len();
        self.players.retain(|player| player.id != player_id);
        let after_player_lens = self.players.len();
        if before_player_lens - after_player_lens == 1 {
            self.state = State::queue;
            false
        } else {
            true
        }
    }

    pub fn red_players(&self) -> Vec<&Player> {
        let mut red_team_players = Vec::with_capacity(5);

        for i in 0..10 {
            if self.team_bit & (1 << i) == 0 {
                red_team_players.push(&self.players[i]);
            }
        }
        // self.players[0..5].iter().map(|player| player).collect()
        red_team_players
    }

    pub fn blue_players(&self) -> Vec<&Player> {
        let mut blue_team_players = Vec::with_capacity(5);
        
        for i in 0..10 {
            if self.team_bit & (1 << i) != 0 {
                blue_team_players.push(&self.players[i]);
            }
        }
        // self.players[0..5].iter().map(|player| player).collect()
        blue_team_players
    }

    pub fn mut_red_players(&mut self) -> Vec<&mut Player> {
        self.players.iter_mut().enumerate().filter(|(index, _)| { self.team_bit & (1 << index) == 0 }).map(|(_, player)| { player }).collect()
    }

    pub fn mut_blue_players(&mut self) -> Vec<&mut Player> {
        self.players.iter_mut().enumerate().filter(|(index, _)| { self.team_bit & (1 << index) != 0 }).map(|(_, player)| { player }).collect()
    }

    pub fn shuffle_team(&mut self) {
        let mut rng = rand::thread_rng();
        // self.players.shuffle(&mut rng);
        // 팀을 어떻게 섞지?
        
        let total_scores = self.players.iter().fold(0, |acc, player| { player.score + acc });

        let combs: Vec<Vec<&Player>> = self.players.iter().combinations(5).collect();
        
        let mut combs_score: Vec<(i32, i32)> = combs.iter().enumerate().map(|(i, players)| {
            (i as i32, players.iter().fold(0, |acc, player| { acc + player.score }))
        }).collect();
        combs_score.sort_by(|lhs, rhs| { 
            if (total_scores / 2).abs_diff(lhs.1) < (total_scores / 2).abs_diff(rhs.1) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        let mut blue_team_index = (&combs_score[0..(combs_score.len() / 3)]).to_vec();
        blue_team_index.shuffle(&mut rng);
        let blue_team_players_ref = &combs[blue_team_index[0].0 as usize];
        let mut new_team_bit = 0;

        for blue_team_player in blue_team_players_ref {
            for i in 0..10 {
                if blue_team_player.id == self.players[i].id {
                    new_team_bit |= (1 << i);
                    break;
                }
            }
        }
        self.team_bit = new_team_bit;
    }

    pub fn red_win(&mut self) {
        self.state = State::result(false);
    }

    pub fn blue_win(&mut self) {
        self.state = State::result(true);
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: u64,
    pub discord_id: Vec<u64>,
    pub summoner_name: String,
    pub tier: Tier,
    pub score: i32,
    pub win: i32,
    pub lose: i32,
    #[serde(default = "score_deviation")]
    pub score_deviation: i32,
}

fn score_deviation() -> i32 {
    200
}

impl Player {
    pub fn new(id: u64, discord_id: u64, summoner_name: String, tier: Tier) -> Player {
        // let score = Tier::tier_to_init_score(&tier);
        let score = 1200;
        Player {
            id,
            discord_id: vec![discord_id],
            summoner_name,
            tier,
            score: score,
            win: 0,
            lose: 0,
            score_deviation: score_deviation()
        }
    }

    pub fn new_v2(id: u64, discord_id: u64) -> Player {
        Player {
            id,
            discord_id: vec![discord_id],
            summoner_name: format!("<@{}>", discord_id),
            tier: Tier::Challenger,
            score: 1300,
            win: 0,
            lose: 0,
            score_deviation: score_deviation()
        }
    }

    pub fn random_dummy() -> Player {
        let id = rand::thread_rng().gen_range(1..100000000);
        let mut scores = vec![1000, 1100, 1200, 1300, 1400, 1500, 1600];
        let mut rng = rand::thread_rng();
        scores.shuffle(&mut rng);

        Player {
            id,
            discord_id: Vec::new(),
            summoner_name: format!("{}", scores[0]),
            tier: Tier::Diamond(Division::II),
            score: scores[0],
            win: 0,
            lose: 0,
            score_deviation: score_deviation()
        }
    }
}

impl Player {
    pub fn win(&mut self, score: i32, score_deviation: i32) {
        self.win += 1;
        self.score = score;
        self.score_deviation = score_deviation;
    }

    pub fn lose(&mut self, score: i32, score_deviation: i32) {
        self.lose += 1;
        self.score = score;
        self.score_deviation = score_deviation;
    }

    pub fn add_discord_id(&mut self, discord_id: u64) {
        self.discord_id.push(discord_id);
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Tier {
    Iron(Division), Bronze(Division), Silver(Division), Gold(Division), Platinum(Division), Diamond(Division), Master, GrandMaster, Challenger
}

impl Tier {
    pub fn iter_tier_name() -> [&'static str; 9] {
        ["Iron", "Bronze", "Silver", "Gold", "Platinum", "Diamond", "Master", "GrandMaster", "Challenger"]
    }

    pub fn deserialize_with_tier_division(tier: String, division:i32) -> Option<Tier> {
        let division = match division {
            1 => Division::I,
            2 => Division::II,
            3 => Division::III,
            4 => Division::IV,
            _ => return None
        };
        match tier.as_str() {
            "Iron" => Some(Tier::Iron(division)),
            "Bronze" => Some(Tier::Bronze(division)),
            "Silver" => Some(Tier::Silver(division)),
            "Gold" => Some(Tier::Gold(division)),
            "Platinum" => Some(Tier::Platinum(division)),
            "Diamond" => Some(Tier::Diamond(division)),
            "Master" => Some(Tier::Master),
            "GrandMaster" => Some(Tier::GrandMaster),
            "Challenger" => Some(Tier::Challenger),
            _ => None
        }
    }

    fn tier_to_init_score(&self) -> i32 {
        match self {
            Tier::Iron(division) => {
                match division {
                    Division::IV => 0,
                    Division::III => 100,
                    Division::II => 200,
                    Division::I => 300
                }
            },
            Tier::Bronze(division) => {
                match division {
                    Division::IV => 400,
                    Division::III => 500,
                    Division::II => 600,
                    Division::I => 700
                }
            },
            Tier::Silver(division) => {
                match division {
                    Division::IV => 800,
                    Division::III => 900,
                    Division::II => 1000,
                    Division::I => 1100
                }
            },
            Tier::Gold(division) => {
                match division {
                    Division::IV => 1200,
                    Division::III => 1300,
                    Division::II => 1400,
                    Division::I => 1500
                }
            },
            Tier::Platinum(division) => {
                match division {
                    Division::IV => 1600,
                    Division::III => 1700,
                    Division::II => 1800,
                    Division::I => 1900
                }
            },
            Tier::Diamond(division) => {
                match division {
                    Division::IV => 2000,
                    Division::III => 2100,
                    Division::II => 2200,
                    Division::I => 2300
                }
            },
            Tier::Master => 2400,
            Tier::GrandMaster => 2600,
            Tier::Challenger => 2800
        }        
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Division {
    I, II, III, IV   
}


mod test {
    use glicko2::{GlickoRating, GameResult};

    use super::*;

    // create game and add player to 10 player, then state will be ready test
    #[test]
    fn test_game_ready() {
        // Given
        let mut game = Game::new(1, Player::random_dummy());
        let mut game_state_result = vec![game.state];

        // When
        for i in 2..=10 {
            game.add_player(Player::random_dummy());
            game_state_result.push(game.state);
        }

        // Then
        let mut expected = vec![State::queue; 9];
        expected.push(State::ready);
        assert_eq!(game_state_result, expected);
    }

    // create game and add player to 10 player, then remove player 10, then state will be queue test
    #[test]
    fn test_game_queue() {
        // Given
        let mut game = Game::new(1, Player::random_dummy());
        let mut game_state_result = vec![game.state];

        // When
        for i in 2..=10 {
            game.add_player(Player::random_dummy());
            game_state_result.push(game.state);
        }
        game.remove_player(game.players.last().unwrap().id);
        game_state_result.push(game.state);
        game.add_player(Player::random_dummy());
        game_state_result.push(game.state);

        // Then
        let mut expected = vec![State::queue; 9];
        expected.push(State::ready);
        expected.push(State::queue);
        expected.push(State::ready);
        assert_eq!(game_state_result, expected);
    }

    #[test]
    fn test_whenNewGame_Create_redTeam5Players_blueTeam5Players() {
        // Given
        let mut game = Game::new(1, Player::random_dummy());

        // When
        for i in 2..=10 {
            game.add_player(Player::random_dummy());
        }

        let blue_players = game.blue_players();
        blue_players[0];

        // Then
        assert_eq!(game.red_players().len(), game.blue_players().len());
        assert_eq!(game.red_players().len(), 5);
    }

    #[test]
    fn test_whenOneTeamWin_ScoreChanged() {
        // Given
        let mut game = Game::new(1, Player::random_dummy());

        // When
        for i in 2..=10 {
            game.add_player(Player::random_dummy());
        }

        let win = game.red_players();
        let lose = game.blue_players();

        let mut new_win_score = Vec::with_capacity(5);
        let mut new_lose_score = Vec::with_capacity(5);

        let mut results = vec![];
        for j in 0..5 {
            results.push(GameResult::win(GlickoRating {
                value: f64::from(lose[j].score),
                deviation: f64::from(lose[j].score_deviation),
            }));
        }

        for i in 0..5 {
            let before_rating = GlickoRating { 
                value: f64::from(win[i].score), 
                deviation: f64::from(win[i].score_deviation) 
            };

            let new_rating: GlickoRating = glicko2::new_rating(before_rating.into(), &results, 0.5).into();
            new_win_score.push(new_rating);
        }

        let mut results = vec![];
        for j in 0..5 {
            results.push(GameResult::loss(GlickoRating {
                value: f64::from(win[j].score),
                deviation: f64::from(win[j].score_deviation),
            }));
        }

        for i in 0..5 {
            let before_rating = GlickoRating { 
                value: f64::from(lose[i].score), 
                deviation: f64::from(lose[i].score_deviation) 
            };

            let new_rating: GlickoRating = glicko2::new_rating(before_rating.into(), &results, 0.5).into();
            new_lose_score.push(new_rating);
        }

        // We are converting the result of new_rating to a GlickoRating immediately, throwing away the
        // benefits of Glicko2 over Glicko for the sake of matching the example in the glicko2 pdf.
        // In a real application, you'd likely want to save the Glicko2Rating and convert to
        // GlickoRating for display purposes only.

        win.iter().for_each(|new_player| {
        println!(
            "Winner rating value: {} New rating deviation: {}",
            new_player.score,
            new_player.score_deviation
        );

        });
            
        
        new_win_score.iter().for_each(|new_rating| {
        println!(
            "Winner rating value: {} New rating deviation: {}",
            new_rating.value,
            new_rating.deviation
        );
        });

        lose.iter().for_each(|new_player| {
            println!(
                "Winner rating value: {} New rating deviation: {}",
                new_player.score,
                new_player.score_deviation
            );
    
            });
            
        new_lose_score.iter().for_each(|new_rating| {
            println!(
                "Lose rating value: {} New rating deviation: {}",
                new_rating.value,
                new_rating.deviation
            );
            });            
    }

    #[test]
    fn test_whenGameIsSet_thenGetSomeRedPlayerRecursively_thenGetSameOrder() {
        // Given
        let mut game = Game::new(1, Player::random_dummy());

        // When
        for i in 2..=10 {
            game.add_player(Player::random_dummy());
        }

        let red_players = game.red_players();
        let red_players_id = red_players.iter().map(|player| player.id).collect::<Vec<u64>>();
        let mut_red_players = &game.mut_red_players();

        // Then
        assert_eq!(red_players_id, mut_red_players.iter().map(|player| player.id).collect::<Vec<u64>>());
    }
}