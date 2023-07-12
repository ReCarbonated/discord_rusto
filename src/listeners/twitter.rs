use std::env;

use super::generic::message_fixer;
use super::Listener;
use lazy_static::lazy_static;
use regex::Regex;
use serenity::client::Context;
use serenity::model::channel::Message;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(\|\|)?http(s)*:\/\/(www\.)?(mobile\.)?(twitter.com)\b([-a-zA-Z0-9()@:%_\+.~#?&//=]{2,}){1,}(\|\|)?",
    ).unwrap();
}

pub async fn handler(ctx: &Context, msg: &Message) {
    message_fixer(ctx, msg, &*RE, "https://vxtwitter.com", 6, (1, 7)).await;
}

pub fn enroll() -> (String, Listener) {
    let switch: bool = env::var("TWITTER_SWITCH")
        .unwrap_or("true".to_string())
        .parse()
        .unwrap();

    (
        "twitter".to_string(),
        Listener {
            name: "twitter".to_string(),
            switch: switch.clone(),
        },
    )
}
