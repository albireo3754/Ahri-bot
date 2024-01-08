use rand::{Rng, seq::SliceRandom};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum State {
    queue, ready, result(bool)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub id: u64,
    pub players: Vec<Player>,
    pub state: State
}


impl Game {
    pub fn new(id: u64, host: Player) -> Game {
        let mut players = vec![host];
        players.reserve(10);
        Game {
            id: id,
            players: players,
            state: State::queue
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
        if self.players.len() == 10 {
            self.state = State::ready;
        }
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
        self.players[0..5].iter().map(|player| player).collect()
    }

    pub fn blue_players(&self) -> Vec<&Player> {
        self.players[5..10].iter().map(|player| player).collect()
    }

    pub fn shuffle_team(&mut self) {
        let mut rng = rand::thread_rng();
        self.players.shuffle(&mut rng);
    }

    pub fn red_win(&mut self) {
        self.state = State::result(true);
    }

    pub fn blue_win(&mut self) {
        self.state = State::result(false);
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
    pub lose: i32
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

impl Player {
    pub fn new(id: u64, discord_id: u64, summoner_name: String, tier: Tier) -> Player {
        let score = Tier::tier_to_init_score(&tier);
        Player {
            id,
            discord_id: vec![discord_id],
            summoner_name,
            tier,
            score: score,
            win: 0,
            lose: 0
        }
    }

    pub fn random_dummy() -> Player {
        let id = rand::thread_rng().gen_range(1..100000000);
        Player::new(id, id, id.to_string(), Tier::Iron(Division::I))
    }

    pub fn add_discord_id(&mut self, discord_id: u64) {
        self.discord_id.push(discord_id);
    }
}

mod test {
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
}