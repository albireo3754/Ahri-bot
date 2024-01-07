use poise::CreateReply;
use ::serenity::builder::{CreateEmbedFooter, CreateEmbed, CreateMessage, CreateAttachment, Builder, CreateButton, CreateInteractionResponseMessage};
use serenity::async_trait;
use serenity::model::Timestamp;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;

use crate::shared::{Context, Error};

#[poise::command(slash_command, rename = "생성", prefix_command)]
pub async fn make_game(
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
      .embed(embed);
      // .to_slash_initial_response(CreateInteractionResponseMessage::new().button(button));
      // .attachment(CreateAttachment::path("./test.png").await.unwrap());
  return builder;
}