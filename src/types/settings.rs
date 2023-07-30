use std::collections::HashMap;
use sqlx::types::Json;

use serde::{Deserialize, Serialize};
// Define settings via nested hashmap probably
#[derive(Serialize, Deserialize, Debug)]
struct Setting {
    #[serde(default)] 
    insta: bool,
    #[serde(default)] 
    pixiv: bool,
    #[serde(default)] 
    misskey: bool,
    #[serde(default)] 
    twitter: bool,
    #[serde(default)] 
    vt_tiktok: bool,
    #[serde(default)] 
    exhentai: bool,
    #[serde(default)]
    admins: Vec<u64>
}

#[derive(sqlx::FromRow)]
struct GuildSetting {
    id: u64,
    setting: Json<Setting>
}


// Deal with first insertion of data.
// Convert into a json.string and then just insert
async fn insert_guild_setting(guild_id: u64, setting: &Setting, pool: &sqlx::MySqlPool) {
    let settings_payload = serde_json::to_string(setting).expect("Failed to somehow encode the settings");

    let _ = sqlx::query!(
        "INSERT INTO `Settings` (`last_edit`, `guild_id`, `setting`) 
        VALUES (NOW(), ?, ?) ON DUPLICATE KEY UPDATE last_edit = NOW(), setting = ?",
        guild_id,
        settings_payload,
        settings_payload
    )
    .execute(pool)
    .await
    .expect("Failed to insert or update the new payload somehow");
}


async fn get_guild_settings(pool: &sqlx::MySqlPool) -> Result<HashMap<u64, sqlx::types::Json<Setting>>, sqlx::Error> {
    let mut output: HashMap<u64, sqlx::types::Json<Setting>> = HashMap::new();

    let sql_query = sqlx::query_as!(GuildSetting, r#"SELECT guild_id as id, setting as "setting: Json<Setting>" FROM Settings"#)
    .fetch_all(pool)
    .await
    .expect("Failed to query DB");

    for single_setting in sql_query {
        output.insert(single_setting.id, single_setting.setting);
    }
    Ok(output)
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