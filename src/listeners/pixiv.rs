use std::env;

use regex::Regex;
use serenity::client::Context;
use serenity::model::channel::Message;

use super::generic::message_fixer;
use super::Listener;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(\|\|)?http(s)*:\/\/(www\.)?(mobile\.)?(pixiv.net)\b(\/\w{2})?(\/artworks\/[\d\/]*)(\|\|)?"
    ).unwrap();
}

pub async fn handler(ctx: &Context, msg: &Message) {
    message_fixer(ctx, msg, &*RE, "https://www.ppxiv.net", 7, (1, 8)).await;
}

pub fn enroll() -> Listener {
    let switch: bool = env::var("PIXIV_SWITCH")
        .unwrap_or("true".to_string())
        .parse()
        .unwrap();

    Listener {
        name: "pixiv".to_string(),
        switch: switch.clone(),
    }
}
