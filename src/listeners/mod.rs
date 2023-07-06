use std::collections::HashSet;

use regex::Regex;
use tokio::sync::Mutex;

mod instagram;
mod misskey;
mod pixiv;
mod tiktok;
mod twitter;
mod generic;

pub struct Switches {
    pub pixiv_switch: bool,
    pub twitter_switch: bool,
    pub insta_switch: bool,
    pub tiktok_switch: bool,
    pub misskey_switch: bool
}

pub struct Handler {
    pub database: sqlx::MySqlPool,
    pub pixiv_regex: Regex,
    pub twitter_regex: Regex,
    pub insta_regex: Regex,
    pub tiktok_regex: Regex,
    pub misskey_regex: Regex,
    pub switches: Mutex<Switches>,
    pub owner: u64,
    pub editors: HashSet<u64>,
    pub client: reqwest::Client
}
