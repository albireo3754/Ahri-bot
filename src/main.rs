use std::env;
mod autocomplete;

use serenity::async_trait;
use ::serenity::builder::{CreateEmbedFooter, CreateEmbed, CreateMessage, CreateAttachment, Builder, CreateButton, CreateInteractionResponseMessage};
use ::serenity::model::Timestamp;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use poise::{serenity_prelude as serenity, CreateReply};
use tokio::time::sleep;


struct Data {
    id: i32,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn message_build() -> CreateReply {
    let footer = CreateEmbedFooter::new("This is a footer");
    let embed = CreateEmbed::new()
        .title("This is a title")
        .description("This is a description")
        .image("attachment://ferris_eyes.png")
        .fields(vec![
            ("This is the first field", "This is a field body", true),
            ("This is the second field", "Both fields are inline", false),
        ])
        .field("This is the third field", "This is not an inline field", false)
        // .field("test", CreateButton::new("test_button")., false);
        .footer(footer)
        // Add a timestamp for the current time
        // This also accepts a rfc3339 Timestamp
        .timestamp(Timestamp::now());
    let enroll = CreateButton::new("test_button");
    let builder = CreateReply::default()
        .content("Hello, World!")
        .embed(embed)
        .to_slash_initial_response(CreateInteractionResponseMessage::new().button(button))
        // .attachment(CreateAttachment::path("./test.png").await.unwrap());
    return builder
}

#[poise::command(slash_command, rename = "등록", prefix_command)]
async fn enroll(
    ctx: Context<'_>
) -> Result<(), Error> {
    let guild = ctx.partial_guild().await.unwrap();
    let members = guild.members(ctx, Option::None, Option::None).await.unwrap();
    let channels = guild.channels(ctx).await.unwrap();
    channels.values().for_each(| channel| {
        let members = channel.members(ctx).unwrap();
        members.iter().for_each(|member| {
            println!("channel: {} member: {}", channel.name(), member.display_name());
        });
    });
    
    members.iter().for_each(|member| {
        println!("member: {}", member.display_name());
    });

    ctx.say(format!(
        "The name of this guild is: {}",
        members.first().unwrap()
    ))
    .await?;

    Ok(())
}

// 게임세션을 하나 만듦
// 세션에다가 플레이어 참가표시를함
// 10명이되면 준비상태가됨
// 준비상태에는 팀원 뽑기 상태가됨
// 팀원 뽑기 상태 이후 재뽑기 or 게임 승리표시가 가능하도록 함
// 준비상태에서도 반복적으로 사람을 뺏다 넣었다는 가능함

#[poise::command(slash_command, rename = "생성", prefix_command)]
async fn make_game(
    ctx: Context<'_>
) -> Result<(), Error> {
    let message = message_build().await;
    let result = ctx.send(message).await;
    // let result = ctx.channel_id().send_message(&ctx, message).await;

    if let Ok(r) = result {
        println!("message :");
    } else {
        println!("Error!");
    }

    Ok(())
}

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
                // enroll(),
                make_game(),
                autocomplete::ahri()
                ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { id: 3 })
            })
        })
        .build();

    tokio::spawn(async move {
        loop {
            sleep(tokio::time::Duration::from_millis(1000)).await;
            println!("1000 ms have elapsed");
        }
    });

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    let result = client.expect("fail to start").start().await;
    
    println!("${}", result.expect_err("error"));
}