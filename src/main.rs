use dotenvy::dotenv;
use chrono_tz::US::Pacific;
use serenity::model::prelude::{ChannelCategory, Channel};

use std::collections::{HashMap, HashSet};
use std::env;
pub mod commands;
pub mod helpers;
pub mod types;
mod listeners;
use listeners::{check_parsers, Handler, Listener};

use commands::*;
use helpers::messages::parse_message;
use serenity::{
    async_trait,
    framework::standard::{
        macros::{command, group},
        {Args, CommandResult, StandardFramework},
    },
    futures::StreamExt,
    model::{
        channel::Message,
        gateway::Ready,
        id::{ChannelId, GuildId},
    },
    prelude::*,
};

use crate::helpers::Pixiv;

struct MessageListener;
struct WebClient;
struct DbPool;
struct Owner;
struct Editors;
struct PixivClientHold;

impl TypeMapKey for MessageListener {
    type Value = HashMap<String, Listener>;
}

impl TypeMapKey for WebClient {
    type Value = reqwest::Client;
}

impl TypeMapKey for DbPool {
    type Value = sqlx::MySqlPool;
}

impl TypeMapKey for Owner {
    type Value = u64;
}

impl TypeMapKey for Editors {
    type Value = HashSet<u64>;
}

impl TypeMapKey for PixivClientHold {
    type Value = helpers::Pixiv;
}

#[group]
#[commands(ping)]
struct General;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.author.bot {
            let guild_name: String;
            match msg.guild(&ctx.cache) {
                Some(guild_match) => {
                    guild_name = guild_match.name;
                }
                _ => {
                    guild_name = "".to_string();
                }
            }

            let channel_name = msg
                .channel_id
                .name(&ctx.cache)
                .await
                .unwrap_or("".to_string());
            let username = msg.author.name.clone();
            
            let time = msg.timestamp.with_timezone(&Pacific).to_rfc3339();
            
            if msg.content.is_empty() {
                println!(
                    "[{}][{}]-[{}]-[{}]: {}",
                    time,
                    guild_name,
                    channel_name,
                    username,
                    &msg.sticker_items
                        .iter()
                        .map(|s| format!("{:?}", s))
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            } else {
                println!(
                    "[{}][{}]-[{}]-[{}]: {}",
                    time, guild_name, channel_name, username, &msg.content
                );
            }
        }

        {
            let listners = ctx.data.read().await;
            let listners: &HashMap<String, Listener> = listners
                .get::<MessageListener>()
                .expect("Expected MessageListener in TypeHash");
            check_parsers(&ctx, &msg, listners).await;
        }

        parse_message(&msg, &ctx).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        match ctx.http.get_channel(192772727281680385).await.unwrap() {
            Channel::Private(channel) => {let _ = channel.say(ctx.http, "Loaded").await;}, 
            _ => {},
        }
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Define all variables via ENV first
    let database_url = env::var("DATABASE_URL").expect("DatabaseURL");
    let owner_string = env::var("OWNER").expect("OWNER");
    let editor_string = env::var("EDITORS").expect("EDITORS");
    let pixiv_token = env::var("PIXIV_TOKEN").expect("PIXIV_TOKEN");
    let owner = owner_string.parse::<u64>().unwrap();
    let editors: HashSet<u64> = editor_string
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    println!("Loading owner: {}", owner);
    println!(
        "Loading editors: {}",
        editors
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );

    // Build database pool
    let database = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Couldn't connect to database");

    // Build event handlers with variables
    let handler = Handler {};


    // Init the framework groups
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP)
        .group(&MATH_GROUP)
        .group(&EMOJIS_GROUP)
        .group(&LISTENERCOMMAND_GROUP)
        .group(&STICKER_GROUP);

    // Login with bot token with Intents
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Build client via junk
    let mut client = Client::builder(token, intents)
        .event_handler(handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    let pixiv_client = Pixiv::new(Option::from(pixiv_token)).expect("Need PIXIV_TOKEN for 18+");

    {
        let mut data = client.data.write().await;
        data.insert::<MessageListener>(listeners::gen_handlers());
        data.insert::<WebClient>(reqwest::Client::new());
        data.insert::<DbPool>(database);
        data.insert::<Editors>(editors);
        data.insert::<Owner>(owner);
        data.insert::<PixivClientHold>(pixiv_client);
    }

    // start listening for events with 1 shard
    if let Err(why) = client.start_shards(1).await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
#[sub_commands(channel_parse, guild_parse)]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
#[aliases("collect")]
#[description("Get things from a channel")]
async fn channel_parse(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    msg.reply(ctx, "Starting Parse!").await?;
    let channel_id = args.single::<u64>()?;

    let channel = ChannelId(channel_id);

    let mut messages = channel.messages_iter(&ctx).boxed();
    while let Some(message_result) = messages.next().await {
        match message_result {
            Ok(message) => parse_message(&message, &ctx).await,
            Err(_error) => {}
        }
    }
    println!("Finished parsing");
    msg.reply(&ctx.http, "done").await?;
    Ok(())
}

#[command]
#[aliases("collect2")]
#[description("Get things from a guild")]
async fn guild_parse(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    msg.reply(ctx, "Starting Parse!").await?;
    let guild_id = args.single::<u64>()?;

    let guild = GuildId(guild_id);
    let guild_map = guild.channels(&ctx.http).await;
    for (channel_id, _) in guild_map? {
        let mut messages = channel_id.messages_iter(&ctx).boxed();
        while let Some(message_result) = messages.next().await {
            match message_result {
                Ok(message) => parse_message(&message, &ctx).await,
                Err(error) => println!("Couldn't access channel: {}", error),
            }
        }
    }
    println!("Finished parsing");
    msg.reply(&ctx.http, "done").await?;

    Ok(())
}
