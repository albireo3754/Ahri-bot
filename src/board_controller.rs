use itertools::Itertools;
use poise::{serenity_prelude as serenity, CreateReply};
use ::serenity::builder::CreateEmbed;

use crate::{shared::{Context, Error}, game::{Tier, Player}};

#[poise::command(slash_command, rename = "보드")]
pub async fn board(
    ctx: Context<'_>
) -> Result<(), Error> {

    let players = ctx.data().player_manager.find_all_player().await;
    ctx.send(board_message_build(players)).await;
    Ok(())
}

fn board_message_build(player: Vec<Player>) -> CreateReply {
  let mut embed = CreateEmbed::new()
    .title(format!("보드"))
    .timestamp(serenity::Timestamp::now());
  let mut builder = CreateReply::default();
  embed = embed.field("",player.iter().sorted_by_key(|player| -player.score).map(|player| { format!("{}({})", player.summoner_name.clone(), player.score) }).collect::<Vec<String>>().join("\n"), false);
  let builder = builder.embed(embed);
  // .to_slash_initial_response(CreateInteractionResponseMessage::new().button(button));
  // .attachment(CreateAttachment::path("./test.png").await.unwrap());
  return builder;
}
