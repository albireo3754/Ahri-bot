use std::env;
mod autocomplete;
mod enroll_controller;
mod make_game_controller;
pub mod game;
pub mod shared;

use serenity::{prelude::*, client::ClientBuilder};
use tokio::time::sleep;
use poise::CreateReply;



#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                enroll_controller::enroll(),
                make_game_controller::make_game(),
                // autocomplete::ahri(),
                make_game_controller::test_reuse_response(),
                make_game_controller::add()
                ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(shared::Data { id: 3 })
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