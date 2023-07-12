use std::env;

use super::generic::message_fixer;
use super::Listener;
use lazy_static::lazy_static;
use regex::Regex;
use serenity::client::Context;
use serenity::model::channel::Message;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(\|\|)?http(s)*:\/\/(www\.)?(mobile\.)?(tiktok.com)\b([-a-zA-Z0-9()@:%_\+.~#?&//=]{1,})(\?.*)?(\|\|)?"
    ).unwrap();
}

pub async fn handler(ctx: &Context, msg: &Message) {
    message_fixer(ctx, msg, &*RE, "https://tiktxk.com", 6, (1, 8)).await;
}

pub fn enroll() -> Listener {
    let tiktok_switch: bool = env::var("TIKTOK_SWITCH")
        .unwrap_or("true".to_string())
        .parse()
        .unwrap();

    Listener {
        name: "tiktok".to_string(),
        switch: tiktok_switch.clone(),
    }
}
