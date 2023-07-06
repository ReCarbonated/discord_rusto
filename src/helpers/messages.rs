use regex::Regex;
use serenity::model::prelude::Message;

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

pub async fn parse_message(msg: &Message, pool: &sqlx::MySqlPool) {
    let user_id = msg.author.id.as_u64();
    insert_user(user_id, pool).await;

    let channel_id = msg.channel_id.as_u64();
    insert_channel(channel_id, pool).await;

    if let Some(guild_id) = msg.guild_id {
        insert_guild(guild_id.as_u64(), pool).await;
    }

    let re = Regex::new(r"<a?:\w*:(?P<id>\d*)>").unwrap();

    for cap in re.captures_iter(&msg.content) {
        if let Some(id_str) = cap.get(1) {
            let emote_id = id_str.as_str().parse::<u64>();
            match emote_id {
                Ok(emote_id) => {
                    insert_emote(&emote_id, pool).await;
                    insert_message(channel_id, user_id, &emote_id, msg, pool).await;
                }
                Err(_error) => {}
            }
        }
    }

    for sticker in &msg.sticker_items {
        let sticker_id = sticker.id.as_u64();
        insert_sticker(sticker_id, pool).await;
        insert_sticker_use(channel_id, user_id, sticker_id, msg, pool).await;
    }
}
