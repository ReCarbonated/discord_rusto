use serenity::{
    builder::CreateEmbed,
    framework::standard::macros::{command, group},
    framework::standard::{Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[group]
#[commands(emoji)]
struct Emojis;

#[command]
#[sub_commands(emoji_guild)]
async fn emoji(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.reply(&ctx.http, "This is the main function!").await?;

    Ok(())
}

#[command]
#[aliases("g")]
#[description("Get things from a guild")]
async fn emoji_guild(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let mut generated_embed = CreateEmbed::default();
    generated_embed.title("Emojis");
    if let Some(guild) = msg.guild(&ctx.cache) {
        for emoji in guild.emojis(&ctx.http).await? {
            generated_embed.field(emoji.name.clone(), format!("{}", emoji), true);
        }
    }

    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("").add_embeds(vec![generated_embed])
        })
        .await;

    if let Err(why) = msg {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
