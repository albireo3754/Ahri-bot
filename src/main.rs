use std::{env, sync::Arc};
mod autocomplete;
mod enroll_controller;
mod make_game_controller;
pub mod game;
pub mod shared;
pub mod player_manager;
pub mod db_manager;
pub mod board_controller;

use serenity::{prelude::*, client::ClientBuilder};
use tokio::time::sleep;

use crate::db_manager::DBManger;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let db_manager = DBManger::new();
    
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    
    let db_ref = Arc::new(db_manager);
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                // enroll_controller::enroll(),
                make_game_controller::make_game(),
                // autocomplete::ahri(),
                // make_game_controller::test_reuse_response(),
                // make_game_controller::add()
                board_controller::board()
                ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(shared::Data { id: 3, player_manager: player_manager::PlayerManager::new(db_ref) })
            })
        })
        .build();

    tokio::spawn(async move {
        loop {
            sleep(tokio::time::Duration::from_millis(1000)).await;
            println!("1000 ms have elapsed");
        }
    });

    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    let result = client.expect("fail to start").start().await;
    
    println!("${}", result.expect_err("error"));
}