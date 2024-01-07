use poise::CreateReply;
use serenity::all::ButtonStyle;
use serenity::builder::{CreateSelectMenu, CreateSelectMenuOption, CreateSelectMenuKind};
use ::serenity::builder::{CreateEmbedFooter, CreateEmbed, CreateMessage, CreateAttachment, Builder, CreateButton, CreateInteractionResponseMessage};
use serenity::{async_trait, builder::CreateActionRow};
use poise::serenity_prelude as serenity;
use serenity::all;
use serenity::model::Timestamp;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;

use crate::{shared::{Context, Error}, game::{Game, Player}};

#[poise::command(slash_command, rename = "생성", prefix_command, reuse_response)]
pub async fn make_game(
    ctx: Context<'_>
) -> Result<(), Error> {
    let message = message_build(Game::new(1, Player { id:1, name: "Miki".into() })).await;
    let result = ctx.send(message).await;
    // let result = ctx.channel_id().send_message(&ctx, message).await;

    while let Some(result) = serenity::ComponentInteractionCollector::new(ctx).filter(move |press| { true }).await {
        println!("{}", result.data.custom_id);
        let result = result.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge).await;
        println!("{:?}", result);
    }

    if let Ok(r) = result {
        println!("message :");
    } else {
        println!("Error!");
    }

    Ok(())
}

async fn message_build(game: Game) -> CreateReply {
    let footer = CreateEmbedFooter::new("This is a footer");
    let embed = CreateEmbed::new()
        .title(format!("방 #{}", game.id))
      // dummy를 5000자가 되도록 반복
        .description(format!("인원: {} / 10\n참여자: [{}]", game.players.len(), game.players.iter().map(|p| p.name.clone()).collect::<Vec<String>>().join(", ")))
        .fields(vec![
            ("블루", "This is\n a field body", true),
            ("레드", "Both fi\nelds are\n in\nline", true),
        ])
        .timestamp(Timestamp::now());

    let join_game_button = CreateButton::new("join_game").label("참가하기").style(ButtonStyle::Primary);
    let leave_game_button = CreateButton::new("leave_game").label("떠나기").style(ButtonStyle::Danger);
    let join_leave_game_row = CreateActionRow::Buttons(vec![join_game_button, leave_game_button]);
    let kick_player_select_menu = CreateSelectMenu::new("custom_id2", CreateSelectMenuKind::String { options: vec![CreateSelectMenuOption::new("포항준기", "Miki")] }).placeholder("추방하기");
    let kick_player_select_menu = CreateActionRow::SelectMenu(kick_player_select_menu);
    let builder = CreateReply::default()
        .content("Hello, World!")
        .embed(embed)
        .components(vec![kick_player_select_menu, join_leave_game_row]);

    
      // .to_slash_initial_response(CreateInteractionResponseMessage::new().button(button));
      // .attachment(CreateAttachment::path("./test.png").await.unwrap());
    return builder;
}


#[poise::command(slash_command, prefix_command)]
pub async fn test_reuse_response(ctx: Context<'_>) -> Result<(), Error> {
    println!("test_reuse_response");
    let image_url = "https://raw.githubusercontent.com/serenity-rs/serenity/current/logo.png";

    let reply = {
        let embed = CreateEmbed::default()
            .description("embed 1")
            .image(image_url);

        let components = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("1")
                .label("button 1")
                .style(serenity::ButtonStyle::Primary),
        ])];

        poise::CreateReply::default()
            .content("message 1")
            .embed(embed)
            .components(components)
    };

    ctx.send(reply).await?;
    println!("sleeping for 2 seconds...");
    // tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    println!("sleeping for 2 seconds...");
    let image_url = "https://raw.githubusercontent.com/serenity-rs/serenity/current/examples/e09_create_message_builder/ferris_eyes.png";
    let reply = {
        let embed = serenity::CreateEmbed::default()
            .description("embed 2")
            .image(image_url);

        let components = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("2")
                .label("button 2")
                .style(serenity::ButtonStyle::Danger),
        ])];

        poise::CreateReply::default()
            .content("message 2")
            .embed(embed)
            .components(components)
    };


    // while let Some(press) = ctx.

    println!("log ctx.send(reply)...");
    ctx.send(reply).await?;
    Ok(())
}

/// Add two numbers
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "First operand"] a: f64,
    #[description = "Second operand"] b: f32,
) -> Result<(), Error> {
    println!("add");
    ctx.say(format!("Result: {}", a + b as f64)).await?;

    Ok(())
}