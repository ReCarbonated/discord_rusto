use dotenvy::dotenv;

use std::collections::HashSet;
use std::env;
pub mod commands;
pub mod helpers;
mod listeners;
use listeners::{Handler, Switches};

use commands::*;
use helpers::messages::parse_message;
use regex::Regex;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult, StandardFramework};
use serenity::futures::StreamExt;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{ChannelId, GuildId};
use serenity::prelude::*;


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

        if (&self.owner == msg.author.id.as_u64()) || (self.editors.contains(msg.author.id.as_u64())){
            match msg.content.as_str() {
                "~toggle pixiv" => {
                    let mut lock = self.switches.lock().await;
                    lock.pixiv_switch = !lock.pixiv_switch;
                },
        
                "~toggle twitter" => {
                    let mut lock = self.switches.lock().await;
                    lock.twitter_switch = !lock.twitter_switch;
                },
                "~toggle insta" => {
                    let mut lock = self.switches.lock().await;
                    lock.insta_switch = !lock.insta_switch;
                }, 
    
                "~toggle tiktok" => {
                    let mut lock = self.switches.lock().await;
                    lock.tiktok_switch = !lock.tiktok_switch;
                },
                "~toggle misskey" => {
                    let mut lock = self.switches.lock().await;
                    lock.misskey_switch = !lock.misskey_switch;
                },
                _ => {}
            }
        }


        let switches = self.switches.lock().await;

        if switches.twitter_switch {
            self.twitter_handler(&ctx, &msg).await;
        }
        
        if switches.pixiv_switch {
            self.pixiv_handler(&ctx, &msg).await;
        }
        
        if switches.insta_switch {
            self.insta_handler(&ctx, &msg).await;
        }

        if switches.tiktok_switch {
            self.tiktok_handler(&ctx, &msg).await;
        }

        if switches.misskey_switch {
            self.misskey_handler(&ctx, &msg).await;
        }
        parse_message(&msg, &self.database).await;
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
    let editors: HashSet<u64> = editor_string.split(",").map(|s| s.parse::<u64>().unwrap()).collect();

    // Define switches via env or default to true
    let pixiv_switch: bool = env::var("PIXIV_SWITCH").unwrap_or("true".to_string()).parse().unwrap();
    let twitter_switch: bool = env::var("TWITTER_SWITCH").unwrap_or("true".to_string()).parse().unwrap();
    let insta_switch: bool = env::var("INSTA_SWITCH").unwrap_or("true".to_string()).parse().unwrap();
    let tiktok_switch: bool = env::var("TIKTOK_SWITCH").unwrap_or("true".to_string()).parse().unwrap();
    let misskey_switch: bool = env::var("TIKTOK_SWITCH").unwrap_or("true".to_string()).parse().unwrap();

    // Build a request engine to reuse later
    let client = reqwest::Client::new();

    println!("Loading owner: {}", owner);
    println!("Loading editors: {}", editors.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));

    // Build database pool
    let database = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Couldn't connect to database");

    // Build regex system
    let pixiv_regex: Regex = Regex::new(
        r"(\|\|)?http(s)*:\/\/(www\.)?(mobile\.)?(pixiv.net)\b(\/\w{2})?(\/artworks\/[\d\/]*)(\|\|)?"
    ).expect("[pixiv_regex]: Failed to compile regex");

    let twitter_regex: Regex = Regex::new(
        r"(\|\|)?http(s)*:\/\/(www\.)?(mobile\.)?(twitter.com)\b([-a-zA-Z0-9()@:%_\+.~#?&//=]{2,}){1,}(\|\|)?",
    ).expect("[twitter_regex]: Failed to compile regex");

    let insta_regex: Regex = Regex::new(
        r"(\|\|)?http(s)*:\/\/(www\.)?(mobile\.)?(instagram.com)\b([-a-zA-Z0-9()@:%_\+.~#?&//=]{1,})(\?.*)?(\|\|)?"
    ).expect("[insta_regex]: Failed to compile regex");

    let tiktok_regex: Regex = Regex::new(
        r"(\|\|)?http(s)*:\/\/(www\.)?(mobile\.)?(tiktok.com)\b([-a-zA-Z0-9()@:%_\+.~#?&//=]{1,})(\?.*)?(\|\|)?"
    ).expect("[tiktok_regex]: Failed to compile regex");

    let misskey_regex: Regex = Regex::new(
        r"(\|\|)?http(s)*:\/\/(www\.)?(mobile\.)?(misskey.io/notes/)\b([-a-zA-Z0-9()@:%_\+.~#?&//=]{1,})(\?.*)?(\|\|)?"
    ).expect("[tiktok_regex]: Failed to compile regex");

    // Define these in a seperate file next time


    // Build switching mutex for logic
    let switches = Mutex::new(Switches {
        pixiv_switch,
        twitter_switch,
        insta_switch,
        tiktok_switch,
        misskey_switch
    });

    // Build event handlers with all your variables
    let handler = Handler {
        database,
        pixiv_regex,
        twitter_regex,
        insta_regex,
        tiktok_regex,
        misskey_regex,
        switches,
        owner,
        editors,
        client
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

    // start listening for events with 2 shards
    if let Err(why) = client.start_shards(2).await {
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
    let pool_workers = args.single::<u32>()?;
    let database_url = env::var("DATABASE_URL").expect("DatabaseURL");
    let database = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(pool_workers)
        .connect(&database_url)
        .await
        .expect("Couldn't connect to database");

    let channel = ChannelId(channel_id);

    let mut messages = channel.messages_iter(&ctx).boxed();
    while let Some(message_result) = messages.next().await {
        match message_result {
            Ok(message) => parse_message(&message, &database).await,
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
    let pool_workers = args.single::<u32>()?;

    let database_url = env::var("DATABASE_URL").expect("DatabaseURL");
    let database = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(pool_workers)
        .connect(&database_url)
        .await
        .expect("Couldn't connect to database");

    let guild = GuildId(guild_id);
    let guild_map = guild.channels(&ctx.http).await;
    for (channel_id, _) in guild_map? {
        let mut messages = channel_id.messages_iter(&ctx).boxed();
        while let Some(message_result) = messages.next().await {
            match message_result {
                Ok(message) => parse_message(&message, &database).await,
                Err(error) => println!("Couldn't access channel: {}", error),
            }
        }
    }
    println!("Finished parsing");
    msg.reply(&ctx.http, "done").await?;

    Ok(())
}
