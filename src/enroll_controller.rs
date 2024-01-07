use crate::shared::{Context, Error};

#[poise::command(slash_command, rename = "등록", prefix_command)]
pub async fn enroll(
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