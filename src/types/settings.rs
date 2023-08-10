use crate::{DbPool, SettingsMap};
use serde::{Deserialize, Serialize};
use serenity::model::guild::Guild;
use serenity::model::prelude::{Message, PartialGuild};
use serenity::{client::Context, model::user::User};
use sqlx::types::Json;
use std::{collections::HashMap, env};
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Listeners {
    #[serde(default)]
    pub insta: bool,
    #[serde(default)]
    pub pixiv: bool,
    #[serde(default)]
    pub misskey: bool,
    #[serde(default)]
    pub twitter: bool,
    #[serde(default)]
    pub vt_tiktok: bool,
    #[serde(default)]
    pub tiktok: bool,
    #[serde(default)]
    pub exhentai: bool,
}

impl Listeners {
    pub fn new() -> Self {
        Listeners {
            insta: true,
            pixiv: true,
            misskey: true,
            twitter: true,
            vt_tiktok: true,
            exhentai: true,
            tiktok: true,
        }
    }
}

impl Default for Listeners {
    fn default() -> Self {
        Listeners {
            insta: true,
            pixiv: true,
            misskey: true,
            twitter: true,
            vt_tiktok: true,
            exhentai: true,
            tiktok: true,
        }
    }
}

// Define settings via nested hashmap probably
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Setting {
    #[serde(default)]
    pub listeners: Listeners,
    #[serde(default)]
    pub admins: Vec<u64>,
    #[serde(default)]
    pub owner: u64,
}

impl Setting {
    pub fn new(owner: u64) -> Self {
        Setting {
            listeners: Listeners::new(),
            admins: Vec::new(),
            owner,
        }
    }

    pub fn is_owner(&self, user_id: &u64) -> bool {
        self.owner == *user_id || env::var("OWNER").unwrap().parse::<u64>().unwrap() == *user_id
    }

    pub fn is_an_admin(&self, user_id: &u64) -> bool {
        self.admins.contains(user_id)
    }

    pub async fn can_edit(&self, ctx: &Context, user: &User, guild: &PartialGuild) -> bool {
        self.is_an_admin(user.id.as_u64())
            || self.is_owner(user.id.as_u64())
            || guild
                .member_permissions(&ctx.http, user)
                .await
                .unwrap()
                .administrator()
    }
}

#[derive(sqlx::FromRow)]
struct GuildSetting {
    id: u64,
    setting: Json<Setting>,
}

// Deal with first insertion of data.
// Convert into a json.string and then just insert
pub async fn insert_guild_setting(guild_id: u64, setting: &Setting, pool: &sqlx::MySqlPool) {
    let settings_payload = serde_json::to_string(setting).unwrap();
    let _ = sqlx::query!(
        "INSERT INTO `Settings` (`last_edit`, `guild_id`, `setting`) 
        VALUES (NOW(), ?, ?) ON DUPLICATE KEY UPDATE last_edit = NOW(), setting = ?",
        guild_id,
        settings_payload,
        settings_payload
    )
    .execute(pool)
    .await
    .unwrap();
}

pub async fn get_guild_settings(
    pool: &sqlx::MySqlPool,
) -> Result<HashMap<u64, Setting>, sqlx::Error> {
    let mut output: HashMap<u64, Setting> = HashMap::new();

    let sql_query = sqlx::query_as!(
        GuildSetting,
        r#"SELECT guild_id as id, setting as "setting: Json<Setting>" FROM Settings"#
    )
    .fetch_all(pool)
    .await
    .expect("Failed to query DB");

    for single_setting in sql_query {
        output.insert(single_setting.id, single_setting.setting.0);
    }
    Ok(output)
}

pub async fn upsert_guild_setting(ctx: Context, guild: Guild, is_new: bool) {
    let mut to_sync = false;
    let mut new_setting = Setting::new(0);
    {
        let settings;
        {
            let data = ctx.data.read().await;
            settings = data.get::<SettingsMap>().unwrap().clone();
        }
        if is_new || !settings.contains_key(guild.id.as_u64()) {
            new_setting = Setting::new(guild.owner_id.0);
            to_sync = true;
        } else {
            // Get the entry of the guild.
            if settings.get(guild.id.as_u64()).unwrap().owner != guild.owner_id.0 {
                new_setting = settings.get(guild.id.as_u64()).unwrap().clone();
                new_setting.owner = guild.owner_id.0;
                to_sync = true;
            } else {
                println!("{:?}", settings.get(guild.id.as_u64()).unwrap());
            }
        }
        if to_sync {
            let pool;
            {
                let data = ctx.data.read().await;
                pool = data.get::<DbPool>().unwrap().clone();
            }

            insert_guild_setting(guild.id.0, &new_setting, &pool).await;
        }
    }

    if to_sync {
        // New closure to remove last state
        let mut data = ctx.data.write().await;
        let settings = data.get_mut::<SettingsMap>().unwrap();
        settings.insert(guild.id.0, new_setting);
    }
}
