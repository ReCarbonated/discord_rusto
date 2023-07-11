use std::{fs::File, io::Write};

use regex::Regex;
use serenity::{
    builder::CreateEmbed,
    framework::standard::macros::{command, group},
    framework::standard::{Args, CommandResult},
    model::{channel::Message},
    prelude::Context,
    utils::parse_emoji,
};

#[group]
#[commands(emoji)]
struct Emojis;

#[command]
#[sub_commands(emoji_guild, emoji_message)]
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

#[command]
#[aliases("m")]
#[description("Get emojis from a message")]
async fn emoji_message(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let message_id = args.single::<u64>()?;
    let channel_id = msg.channel_id.as_u64();

    let emote_message = ctx.http.get_message(*channel_id, message_id).await;
    match emote_message {
        Ok(message) => {
            let re = Regex::new(r"<a?:\w*:(?P<id>\d*)>").unwrap();
            let list_of_ids = re
                .captures_iter(&message.content)
                .map(|e| parse_emoji(e.get(0).unwrap().as_str()))
                .collect::<Vec<_>>();

            let mut file = File::create(format!("/home/carbon/{}.txt", &msg.id)).expect("Unable to create file");
            list_of_ids.iter().for_each(|e| {
                let cloned = e.clone().unwrap();
                file.write(cloned.name.as_bytes()).expect("Unable to write data");
                file.write(" - ".as_bytes()).expect("Unable to write data");
                file.write(cloned.animated.to_string().as_bytes()).expect("Unable to write data");
                file.write(" - ".as_bytes()).expect("Unable to write data");
                file.write(cloned.url().as_bytes()).expect("Unable to write data");
                file.write("\n".as_bytes()).expect("Unable to write data");
            });
            // println!("{:?}", list_of_ids);
        }
        Err(_) => {
            println!("Something fucked up");
        }
    }

    // let mut generated_embed = CreateEmbed::default();

    // generated_embed.title("Emojis");
    // if let Some(guild) = msg.guild(&ctx.cache) {
    //     for emoji in guild.emojis(&ctx.http).await? {
    //         generated_embed.field(emoji.name.clone(), format!("{}", emoji), true);
    //     }
    // }

    // let msg = msg
    //     .channel_id
    //     .send_message(&ctx.http, |m| {
    //         m.content("").add_embeds(vec![generated_embed])
    //     })
    //     .await;

    // if let Err(why) = msg {
    //     println!("Error sending message: {:?}", why);
    // }

    Ok(())
}
