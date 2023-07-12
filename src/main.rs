use dotenvy::dotenv;

use std::collections::HashSet;
use std::env;
pub mod commands;
pub mod helpers;
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

struct MessageListener;
struct WebClient;
struct DbPool;

impl TypeMapKey for MessageListener {
    type Value = Vec<Listener>;
}

impl TypeMapKey for WebClient {
    type Value = reqwest::Client;
}

impl TypeMapKey for DbPool {
    type Value = sqlx::MySqlPool;
}

#[group]
#[commands(ping)]
struct General;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.is_empty() {
            println!(
                "{}",
                &msg.sticker_items
                    .iter()
                    .map(|s| format!("{:?}", s))
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        } else {
            println!("{}", &msg.content);
        }
        if (&self.owner == msg.author.id.as_u64())
            || (self.editors.contains(msg.author.id.as_u64()))
        {
            match msg.content.as_str() {
                "~toggle status" => {
                    self.print_status(&ctx, &msg).await;
                }
                _ => {}
            }
        }
        {
            let listners = ctx.data.read().await;
            let listners: &Vec<Listener> = listners
                .get::<MessageListener>()
                .expect("Expected MessageListener in TypeHash");
            check_parsers(&ctx, &msg, listners).await;
        }

        parse_message(&msg, &ctx).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
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
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Couldn't connect to database");
    // Define these in a seperate file next time

    // Build event handlers with variables
    let handler = Handler { owner, editors };

    let list_of_listeners: Vec<Listener> = {
        let mut collect = Vec::new();
        collect.push(listeners::tiktok::enroll());
        collect.push(listeners::misskey::enroll());
        collect.push(listeners::twitter::enroll());
        collect.push(listeners::instagram::enroll());
        collect.push(listeners::pixiv::enroll());

        collect
    };

    // Init the framework groups
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP)
        .group(&MATH_GROUP)
        .group(&EMOJIS_GROUP);

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

    {
        let mut data = client.data.write().await;
        data.insert::<MessageListener>(list_of_listeners);
        data.insert::<WebClient>(reqwest::Client::new());
        data.insert::<DbPool>(database);
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
