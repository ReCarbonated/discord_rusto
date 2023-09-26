use regex::Regex;
use reqwest::Client;
use serenity::{model::prelude::Message, prelude::Context};
use std::io::Write;
use std::fs::File;
use anyhow::Result;
use std::path::Path;

use crate::{DbPool, WebClient};

async fn insert_user(user_id: &u64, pool: &sqlx::MySqlPool) {
    sqlx::query!(
        "INSERT INTO Users (create_time, discord_id) SELECT ?, ? FROM (select 1) as a WHERE NOT EXISTS (SELECT discord_id FROM Users WHERE discord_id = ?)",
        sqlx::types::chrono::Utc::now(),
        user_id,
        user_id,
    )
    .execute(pool) // < Where the command will be executed
    .await
    .unwrap();
}

async fn insert_channel(channel_id: &u64, pool: &sqlx::MySqlPool) {
    sqlx::query!(
        "INSERT INTO Channels (create_time, discord_id) SELECT ?, ? FROM (select 1) as a WHERE NOT EXISTS (SELECT discord_id FROM Channels WHERE discord_id = ?)",
        sqlx::types::chrono::Utc::now(),
        channel_id,
        channel_id,
    )
    .execute(pool) // < Where the command will be executed
    .await
    .unwrap();
}

async fn insert_guild(guild_id: &u64, pool: &sqlx::MySqlPool) {
    sqlx::query!(
        "INSERT INTO Guilds (create_time, discord_id) SELECT ?, ? FROM (select 1) as a WHERE NOT EXISTS (SELECT discord_id FROM Guilds WHERE discord_id = ?)",
        sqlx::types::chrono::Utc::now(),
        guild_id,
        guild_id,
    )
    .execute(pool) // < Where the command will be executed
    .await
    .unwrap();
}

async fn insert_emote(emote_id: &u64, pool: &sqlx::MySqlPool) {
    sqlx::query!(
        "INSERT INTO Emotes (create_time, discord_id) SELECT ?, ? FROM (select 1) as a WHERE NOT EXISTS (SELECT discord_id FROM Emotes WHERE discord_id = ?)",
        sqlx::types::chrono::Utc::now(),
        emote_id,
        emote_id,
    )
    .execute(pool) // < Where the command will be executed
    .await
    .unwrap();
}

async fn insert_sticker(sticker_id: &u64, pool: &sqlx::MySqlPool) {
    sqlx::query!(
        "INSERT INTO Stickers (create_time, discord_id) SELECT ?, ? FROM (select 1) as a WHERE NOT EXISTS (SELECT discord_id FROM Stickers WHERE discord_id = ?)",
        sqlx::types::chrono::Utc::now(),
        sticker_id,
        sticker_id,
    )
    .execute(pool)
    .await
    .unwrap();
}

async fn insert_sticker_use(
    channel_id: &u64,
    user_id: &u64,
    sticker_id: &u64,
    msg: &Message,
    pool: &sqlx::MySqlPool,
) {
    sqlx::query!(
        "INSERT INTO
        Sticker_Use (
            channel_id,
            user_id,
            sticker_id,
            create_time,
            message_time,
            discord_id
        )
    SELECT (
            SELECT id
            FROM Channels
            WHERE
                discord_id = ?
        ), (
            SELECT id
            FROM Users
            WHERE
                discord_id = ?
        ), (
            SELECT id
            FROM Stickers
            WHERE
                discord_id = ?
        ),
        ?,
        ?,
        ?
    FROM DUAL
    WHERE NOT EXISTS(
            SELECT channel_id
            FROM Sticker_Use
            WHERE channel_id = (
                    SELECT id
                    FROM Channels
                    WHERE
                        discord_id = ?
                )
                AND user_id = (
                    SELECT id
                    FROM Users
                    WHERE
                        discord_id = ?
                )
                AND sticker_id = (
                    SELECT id
                    FROM Stickers
                    WHERE
                        discord_id = ?
                )
            LIMIT 1
        )",
        channel_id,
        user_id,
        sticker_id,
        sqlx::types::chrono::Utc::now(),
        &msg.timestamp.naive_utc(),
        &msg.id.as_u64(),
        channel_id,
        user_id,
        sticker_id
    )
    .execute(pool)
    .await
    .unwrap();
}

async fn insert_message(
    channel_id: &u64,
    user_id: &u64,
    emote_id: &u64,
    msg: &Message,
    pool: &sqlx::MySqlPool,
) {
    sqlx::query!(
        "INSERT INTO
        Messages (
            channel_id,
            user_id,
            emote_id,
            create_time,
            message_time,
            discord_id
        )
    SELECT (
                SELECT id
                FROM Channels
                WHERE
                    discord_id = ?
            ), (
                SELECT id
                FROM Users
                WHERE
                    discord_id = ?
            ), (
                SELECT id
                FROM Emotes
                WHERE
                    discord_id = ?
            ),
            ?,
            ?,
            ?
    FROM DUAL
    WHERE NOT EXISTS(
            SELECT channel_id
            FROM Messages
            WHERE channel_id = (
                    SELECT id
                    FROM Channels
                    WHERE
                        discord_id = ?
                )
                AND user_id = (
                    SELECT id
                    FROM Users
                    WHERE
                        discord_id = ?
                )
                AND emote_id = (
                    SELECT id
                    FROM Emotes
                    WHERE
                        discord_id = ?
                ) LIMIT 1
        )",
        channel_id,
        user_id,
        emote_id,
        sqlx::types::chrono::Utc::now(),
        &msg.timestamp.naive_utc(),
        &msg.id.as_u64(),
        channel_id,
        user_id,
        emote_id,
    )
    .execute(pool)
    .await
    .unwrap();
}

pub async fn sent_message_to_db(message_id: &u64, ref_message_id: &u64, pool: &sqlx::MySqlPool) {
    sqlx::query!(
        "INSERT INTO SentMessages (message_id, referenced_id, create_time) VALUES (?, ?, ?)",
        message_id,
        ref_message_id,
        sqlx::types::chrono::Utc::now(),
    )
    .execute(pool)
    .await
    .unwrap();
}

pub async fn message_interacted_by_bot(message_id: &u64, pool: &sqlx::MySqlPool) -> sqlx::Result<u64> {
    let res = sqlx::query!(
        "SELECT * FROM SentMessages WHERE referenced_id = ? LIMIT 1",
        message_id
    ).fetch_one(pool).await;

    match res {
        Ok(bot_msg) => Ok(bot_msg.message_id),
        Err(err) => Err(err)
    }

}

pub async fn parse_message(msg: &Message, ctx: &Context) {
    let user_id = msg.author.id.as_u64();
    let pool = {
        let data = ctx.data.read().await;
        data
            .get::<DbPool>()
            .expect("Expected DbPool in TypeMap").clone()
    };

    let client = {
        let data = ctx.data.read().await;
        data
            .get::<WebClient>()
            .expect("Expected DbPool in TypeMap").clone()
    };


    insert_user(user_id, &pool).await;

    let channel_id = msg.channel_id.as_u64();
    insert_channel(channel_id, &pool).await;

    if let Some(guild_id) = msg.guild_id {
        insert_guild(guild_id.as_u64(), &pool).await;
    }

    let re = Regex::new(r"<(a?):\w*:(?P<id>\d*)>").unwrap();

    for cap in re.captures_iter(&msg.content) {
        if let Some(id_str) = cap.get(2) {
            let emote_id = id_str.as_str().parse::<u64>();
            match emote_id {
                Ok(emote_id) => {
                    let ext = {
                        match cap.get(1) {
                            Some(contains) => {
                                match contains.is_empty() {
                                    true => {"png"},
                                    false => {"gif"},
                                }
                            },
                            None => "png",
                        }
                    };
                    insert_emote(&emote_id, &pool).await;
                    insert_message(channel_id, user_id, &emote_id, msg, &pool).await;
                    match download_emote(&client, &emote_id, ext).await {
                        Ok(_) => println!("Was ok in downloading emote"),
                        Err(_) => println!("Was not ok in downloading emote"),
                    }
                }
                Err(_error) => {}
            }
        }
    }

    for sticker in &msg.sticker_items {
        let sticker_id = sticker.id.as_u64();
        {
            insert_sticker(sticker_id, &pool).await;
            insert_sticker_use(channel_id, user_id, sticker_id, msg, &pool).await;
        }
    }
}

async fn download_emote(client: &Client, emote_id: &u64, ext: &str) -> Result<()> {
    let filename = format!("/emote/{}.{}", emote_id, ext);
    // let filename = format!("/home/carbon/emote/{}.{}", emote_id, ext);
    println!("{}", filename);
    match Path::new(&filename).exists() {
        true => {},
        false => {
            let mut out = File::create(filename)?;
            let url = format!("https://cdn.discordapp.com/emojis/{}.{}", emote_id, ext);
            println!("{}", url);
            let resp = fetch_bytes(client, url.as_str()).await?;
            out.write_all(&resp)?;
        },
    }
    Ok(())
}

async fn fetch_bytes(client: &Client, url: &str) -> Result<Vec<u8>> {
    let data = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;

    Ok(data.to_vec())
}