use rand::{Rng, seq::SliceRandom};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    queue, ready, result(bool)
}

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

    pub fn remove_player(&mut self, player_id: u64) {
        self.players.retain(|player| player.id != player_id);
        self.state = State::queue;
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

pub struct Player {
    pub id: u64,
    pub name: String
}

impl Player {
    pub fn random_dummy() -> Player {
        let id = rand::thread_rng().gen_range(1..100000000);
        Player {
            id,
            name: id.to_string()
        }
    }
}

mod test {
    use super::*;

    // create game and add player to 10 player, then state will be ready test
    #[test]
    fn test_game_ready() {
        // Given
        let mut game = Game::new(1, Player{id: 1, name: "host".to_string()});
        let mut game_state_result = vec![game.state];

        // When
        for i in 2..=10 {
            game.add_player(Player{id: i, name: format!("player{}", i)});
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
        let mut game = Game::new(1, Player{id: 1, name: "host".to_string()});
        let mut game_state_result = vec![game.state];

        // When
        for i in 2..=10 {
            game.add_player(Player{id: i, name: format!("player{}", i)});
            game_state_result.push(game.state);
        }
        game.remove_player(10);
        game_state_result.push(game.state);
        game.add_player(Player{id: 10, name: "player10".to_string()});
        game_state_result.push(game.state);

        // Then
        let mut expected = vec![State::queue; 9];
        expected.push(State::ready);
        expected.push(State::queue);
        expected.push(State::ready);
        assert_eq!(game_state_result, expected);
    }
}