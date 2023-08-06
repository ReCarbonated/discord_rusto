use serenity::{
    framework::standard::macros::{command, group},
    framework::standard::{Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[group]
#[commands(sticker)]
struct Sticker;

#[command]
#[sub_commands(sticker_message)]
async fn sticker(_ctx: &Context, _msg: &Message, _args: Args) -> CommandResult {
    Ok(())
}

#[command]
#[aliases("m", "message")]
#[description("Get sticker from a message")]
async fn sticker_message(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let message_id = args.single::<u64>()?;
    let channel_id = args.single::<u64>().unwrap_or(msg.channel_id.0);

    let sticker_message = ctx.http.get_message(channel_id, message_id).await;

    match sticker_message {
        Ok(message) => {
            msg.reply(
                &ctx.http,
                format!(
                    "{}",
                    message
                        .sticker_items
                        .iter()
                        .map(|i| i.image_url().unwrap_or("".to_string()))
                        .collect::<Vec<_>>()
                        .join(" ")
                ),
            )
            .await?;
        }
        Err(_) => {
            eprintln!("Failed to parse message to get sticker");
            msg.reply(&ctx.http, "Failed to grab message with supplied id and/or channel_id, might need to specify them.").await.expect("Something went wrong");
        }
    }

    Ok(())
}
