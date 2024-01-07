use std::env;
mod autocomplete;
mod enroll;
mod make_game;
pub mod shared;

use serenity::{prelude::*, client::ClientBuilder};
use tokio::time::sleep;
use poise::CreateReply;

// 게임세션을 하나 만듦
// 세션에다가 플레이어 참가표시를함
// 10명이되면 준비상태가됨
// 준비상태에는 팀원 뽑기 상태가됨
// 팀원 뽑기 상태 이후 재뽑기 or 게임 승리표시가 가능하도록 함
// 준비상태에서도 반복적으로 사람을 뺏다 넣었다는 가능함



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
                enroll::enroll(),
                make_game::make_game(),
                autocomplete::ahri()
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