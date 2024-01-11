use core::fmt;
use std::rc::Rc;

use itertools::Itertools;
use poise::CreateReply;
use rand::Rng;
use serenity::all::ButtonStyle;
use ::serenity::all::UserId;
use serenity::builder::{CreateSelectMenu, CreateSelectMenuOption, CreateSelectMenuKind};
use ::serenity::builder::{CreateEmbedFooter, CreateEmbed, CreateMessage, CreateAttachment, Builder, CreateButton, CreateInteractionResponseMessage};
use serenity::{async_trait, builder::CreateActionRow};
use poise::serenity_prelude as serenity;
use serenity::all;
use serenity::model::Timestamp;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;

use crate::player_manager::PlayerManager;
use crate::{game, player_manager};
use crate::shared::{Data, CustomError};
use crate::{shared::{Context, Error}, game::{Game, Player}};

async fn get_player_from_discord_context(ctx: &Context<'_>, discord_user_id: UserId) -> Result<Player, Error> {
    let player = ctx.data().player_manager.find_player_with_discord_user_id(discord_user_id.get()).await;
    if player.is_none() {
        let player = ctx.data().player_manager.register_player_v2(discord_user_id.get()).await;
        return Ok(player);
    } else {
        return Ok(player.unwrap());
    }
}

#[poise::command(slash_command, rename = "생성")]
pub async fn make_game(
    ctx: Context<'_>
) -> Result<(), Error> {
    let player = get_player_from_discord_context(&ctx, ctx.author().id).await?;
    
    let mut game = ctx.data().player_manager.generate_game(player).await;
    let message = message_build(&game);
    let result = ctx.send(message).await;
    let game_id = game.id;

    while let Some(interaction) = serenity::ComponentInteractionCollector::new(ctx).filter(move |interaction| { interaction.data.custom_id.starts_with(format!("{}.", game_id).as_str()) }).await {
        let custom_id_without_game_id = interaction.data.custom_id.strip_prefix(format!("{}.", game_id).as_str()).unwrap_or(""); 
        match custom_id_without_game_id {
            "join_game" => {
                let player = get_player_from_discord_context(&ctx, interaction.user.id).await?;
                if !game.add_player(player) {
                    ctx.say("이미 등록된 유저입니다.").await;
                    continue;
                }
            } 
            "leave_game" => {
                let player = get_player_from_discord_context(&ctx, interaction.user.id).await?;
                game.remove_player(player.id);
                println!("leave game {}", interaction.data.custom_id);
            } 
            "kick" => {
                ctx.say("준비중인 기능입니다").await;
                continue;
                // let data = &interaction.data.kind;
                // match data {
                //     serenity::ComponentInteractionDataKind::StringSelect { values } => {
                //         // TODO: - 왜 value array로 받는지 조사해야함
                //         println!("kick {}", values[0]);
                //     }
                //     _ => {
                //         println!("잘못됨됨");
                //     }
                // }
            }
            "test" => {
                for i in 1..10 {
                    game.add_player(Player::random_dummy());
                }
            }
            "red_win" => {
                game.red_win();
                ctx.data().player_manager.end_game(game.clone()).await;
            }
            "blue_win" => {
                game.blue_win();
                ctx.data().player_manager.end_game(game.clone()).await;
            }
            "team_shuffle" => {
                game.shuffle_team();
            }
            _ => {
                ctx.say("모몬가").await;
                continue;
            }
        }

        let message = message_build(&game);
        let response_message = CreateInteractionResponseMessage::default().embeds(message.embeds).components(message.components.unwrap_or(vec![]));
        // let editmessage = serenity::EditInteractionResponse::default().embeds(message.embeds).components(message.components.unwrap_or(vec![]);
        let result = interaction.create_response(ctx, serenity::CreateInteractionResponse::UpdateMessage(response_message)).await;
        println!("request: {}\nresult: {:?}", interaction.data.custom_id, result);
    }

    if let Ok(r) = result {
        println!("message :");
    } else {
        println!("Error!");
    }

    Ok(())
}

fn message_build(game: &Game) -> CreateReply {
    let footer = CreateEmbedFooter::new("This is a footer");
    let mut embed = CreateEmbed::new()
        .title(format!("방 #{}", game.id))
        .timestamp(Timestamp::now());
    let mut builder = CreateReply::default();

    if let game::State::result(red_win) = game.state {
        let red_names = game.red_players().iter().map(|player| { player.summoner_name.clone() }).collect::<Vec<String>>().join("\n");
        let blue_names = game.blue_players().iter().map(|player| { player.summoner_name.clone() }).collect::<Vec<String>>().join("\n");
        embed = embed.fields(vec![("레드", red_names, true), ("블루", blue_names, true)]);
        if red_win {
            embed = embed.description("레드팀 승리!").colour(serenity::Colour::RED);
        } else {
            embed = embed.description("블루팀 승리!").colour(serenity::Colour::BLUE);
        }
    } else if game.players.len() == 10 {
        let red_names = game.red_players().iter().sorted_by_key(|x| -x.score).map(|player| { format!("{}({})", player.summoner_name.clone(), player.score) }).collect::<Vec<String>>().join("\n");
        let blue_names = game.blue_players().iter().sorted_by_key(|x| -x.score).map(|player| { format!("{}({})", player.summoner_name.clone(), player.score) }).collect::<Vec<String>>().join("\n");
        embed = embed.fields(vec![("레드", red_names, true), ("블루", blue_names, true)]);
        
        let red_win = CreateButton::new(format!("{}.red_win", game.id)).label("레드팀 승").style(ButtonStyle::Danger);
        let blue_win = CreateButton::new(format!("{}.blue_win", game.id)).label("블루팀 승").style(ButtonStyle::Primary);
        let team_shuffle = CreateButton::new(format!("{}.team_shuffle", game.id)).label("팀 섞기").style(ButtonStyle::Secondary);
        let leave_game_button = CreateButton::new(format!("{}.leave_game", game.id)).label("떠나기").style(ButtonStyle::Danger);
        let win_row = CreateActionRow::Buttons(vec![red_win, blue_win, team_shuffle]);

        let join_game_button = CreateButton::new(format!("{}.join_game", game.id)).label("참가하기").style(ButtonStyle::Primary).disabled(true);
        let leave_game_button = CreateButton::new(format!("{}.leave_game", game.id)).label("떠나기").style(ButtonStyle::Danger);
        let join_leave_game_row = CreateActionRow::Buttons(vec![join_game_button, leave_game_button]);
        let kick_player_select_menu = CreateSelectMenu::new(format!("{}.kick", game.id), CreateSelectMenuKind::String { options: vec![CreateSelectMenuOption::new("포항준기", "Miki")] }).placeholder("추방하기");
        let kick_player_select_menu = CreateActionRow::SelectMenu(kick_player_select_menu);
        builder = builder.components(vec![win_row, join_leave_game_row, kick_player_select_menu]);
    } else {
        embed = embed.description(format!("인원: {} / 10\n참여자: [{}]", game.players.len(), game.players.iter().map(|player| player.summoner_name.clone()).collect::<Vec<String>>().join(", ")));

        let join_game_button = CreateButton::new(format!("{}.join_game", game.id)).label("참가하기").style(ButtonStyle::Primary);
        let leave_game_button = CreateButton::new(format!("{}.leave_game", game.id)).label("떠나기").style(ButtonStyle::Danger);
        let test_game_button = CreateButton::new(format!("{}.test", game.id)).label("더미 테스트용").style(ButtonStyle::Danger);
        let join_leave_game_row = CreateActionRow::Buttons(vec![join_game_button, leave_game_button]);
        let test_game_row = CreateActionRow::Buttons(vec![test_game_button]);
        let kick_player_select_menu = CreateSelectMenu::new(format!("{}.kick", game.id), CreateSelectMenuKind::String { options: vec![CreateSelectMenuOption::new("포항준기", "Miki")] }).placeholder("추방하기");
        let kick_player_select_menu = CreateActionRow::SelectMenu(kick_player_select_menu);
        builder = builder.components(vec![
            join_leave_game_row,
            test_game_row,
            kick_player_select_menu
            ]);
    }

    let builder = builder.embed(embed);
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