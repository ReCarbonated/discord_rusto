use serde::{Deserialize, Serialize};
// Define settings via nested hashmap probably

#[derive(Serialize, Deserialize)]
struct GuildSetting {
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
    exhentai: bool
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