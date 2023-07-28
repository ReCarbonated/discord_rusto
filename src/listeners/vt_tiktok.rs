use std::env;

use super::generic::message_fixer;
use super::Listener;
use lazy_static::lazy_static;
use regex::Regex;
use serenity::client::Context;
use serenity::model::channel::Message;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(\|\|)?http(s)*:\/\/(vt\.)?(tiktok.com)\b([-a-zA-Z0-9()@:%_\+.~#?&//=]{1,})(\?.*)?(\|\|)?"
    )
    .unwrap();
}

pub async fn handler(ctx: &Context, msg: &Message) {
    message_fixer(ctx, msg, &*RE, "https://vt.tiktxk.com", 5, (1, 7), true).await;
}

pub fn enroll() -> (String, Listener) {
    let tiktok_switch: bool = env::var("TIKTOK_SWITCH")
        .unwrap_or("true".to_string())
        .parse()
        .unwrap();

    (
        "vt_tiktok".to_string(),
        Listener {
            name: "vt_tiktok".to_string(),
            switch: tiktok_switch.clone(),
        },
    )
}
