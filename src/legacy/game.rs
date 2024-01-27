use serde::{ser::SerializeSeq, Deserialize, Serialize, Serializer};

use crate::game::State;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameV0 {
    pub id: u64,
    #[serde(serialize_with = "player_ids")]
    pub players: Vec<PlayerV0>,
    pub state: State,
    pub team_bit: i32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerV0 {
    pub id: u64,
    pub discord_id: Vec<u64>,
    pub summoner_name: String,
    pub score: i32,
    pub win: i32,
    pub lose: i32,
    #[serde(default = "score_deviation")]
    pub score_deviation: i32,
}

fn score_deviation() -> i32 {
    200
}

fn player_ids<S>(players: &Vec<PlayerV0>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
  let mut seq = serializer.serialize_seq(Some(players.len()))?;
  for element in players {
      seq.serialize_element(&element.id)?;
  }
  seq.end()
}
