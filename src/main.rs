use std::{env, sync::Arc, net::{TcpListener, TcpStream}, io::{Write, Read}};
mod autocomplete;
mod enroll_controller;
mod make_game_controller;
pub mod game;
pub mod shared;
pub mod player_manager;
pub mod db;
pub mod board_controller;
pub mod legacy;

use serenity::{prelude::*, client::ClientBuilder};
use tokio::time::sleep;

use crate::db::{inmemory_db::InMemoryDBManger, supabase_db::SupabaseDBManager};


#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();

    // let db_manager = InMemoryDBManger::new();
    let db_manager: SupabaseDBManager = SupabaseDBManager::new().await;
    
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

    tokio::spawn(async move {
        let app = axum::Router::new()
        .route("/", axum::routing::get(|| async { "Hello, world!" }))
        .route("/health", axum::routing::get(|| async { axum::http::StatusCode::NO_CONTENT }));
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
        axum::serve(listener, app)
            .await
            .unwrap();
    });

    println!("server will start");
    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    let result = client.expect("fail to start").start().await;
    
    println!("${:?}", result);
}