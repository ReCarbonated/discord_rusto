use std::collections::HashMap;
use sqlx::types::Json;
use serenity::client::Context;
use serenity::model::guild::Guild;
use serde::{Deserialize, Serialize};
use crate::{SettingsMap, DbPool};
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
        Listeners { insta: true, pixiv: true, misskey: true, twitter: true, vt_tiktok: true, exhentai: true, tiktok: true }
    }
}

impl Default for Listeners {
    fn default() -> Self {
        Listeners { insta: true, pixiv: true, misskey: true, twitter: true, vt_tiktok: true, exhentai: true, tiktok: true }  
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
    pub owner: u64
}

impl Setting {
    pub fn new(owner: u64) -> Self {
        Setting { listeners: Listeners::new(), admins: Vec::new(), owner}
    }
}

#[derive(sqlx::FromRow)]
struct GuildSetting {
    id: u64,
    setting: Json<Setting>
}


// Deal with first insertion of data.
// Convert into a json.string and then just insert
pub async fn insert_guild_setting(guild_id: u64, setting: &Setting, pool: &sqlx::MySqlPool) {
    println!("{:?}", setting);
    let settings_payload = serde_json::to_string(setting).unwrap();
    println!("{:?}", settings_payload);
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


pub async fn get_guild_settings(pool: &sqlx::MySqlPool) -> Result<HashMap<u64, Setting>, sqlx::Error> {
    let mut output: HashMap<u64, Setting> = HashMap::new();

    let sql_query = sqlx::query_as!(GuildSetting, r#"SELECT guild_id as id, setting as "setting: Json<Setting>" FROM Settings"#)
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
        let data = ctx.data.read().await;
        let settings = data.get::<SettingsMap>().unwrap();
        if is_new || !settings.contains_key(guild.id.as_u64()){
            new_setting = Setting::new(guild.owner_id.0);
            to_sync = true;
        } else {
            println!("Got into entry");
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
            println!("Got into sync");
            let pool = data.get::<DbPool>().unwrap();
            println!("Got past pool");
            println!("{:?}", new_setting);
            insert_guild_setting(guild.id.0, &new_setting, pool).await;
            println!("finished syncing");
        }
    }

    if to_sync {
        println!("Sync 2");
        // New closure to remove last state
        let mut data = ctx.data.write().await;
        let settings = data.get_mut::<SettingsMap>().unwrap();
        settings.insert(guild.id.0, new_setting);
    }        
}











// use std::env;
// use dotenvy::dotenv;
// use serde::{Deserialize, Serialize};
// use sqlx::types::Json;
// // Define settings via nested hashmap probably

// #[derive(Serialize, Deserialize, Debug)]
// struct Setting {
//     #[serde(default)] 
//     insta: bool,
//     #[serde(default)] 
//     pixiv: bool,
//     #[serde(default)] 
//     misskey: bool,
//     #[serde(default)] 
//     twitter: bool,
//     #[serde(default)] 
//     vt_tiktok: bool,
//     #[serde(default)] 
//     exhentai: bool
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct GuildSetting {
//     id: u64,
//     setting: Json<Setting>
// }

// #[tokio::main]
// async fn main() {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DatabaseURL");
//     let database = sqlx::mysql::MySqlPoolOptions::new()
//         .max_connections(10)
//         .connect(&database_url)
//         .await
//         .expect("Couldn't connect to database");

//     let id:i64 = 192741437366992896;

//     let res = sqlx::query_as!(GuildSetting, r#"SELECT guild_id as id, setting as "setting: Json<Setting>" FROM Settings WHERE guild_id=?"#, id).fetch_one(&database)
//     .await
//     .unwrap();

//     println!("{:?}", res);
// }