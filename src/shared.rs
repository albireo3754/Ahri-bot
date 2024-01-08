use crate::player_manager::PlayerManager;

pub struct Data {
  pub id: i32,
  pub player_manager: PlayerManager,
} // User data, which is stored and accessible in all command invocations

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;