use std::env;

use lazy_static::lazy_static;
use regex::Regex;
use serenity::client::Context;
use serenity::model::channel::Message;

use super::generic::message_fixer;
use super::Listener;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(\|\|)?http(s)*:\/\/(www\.)?(mobile\.)?(instagram.com)\b([-a-zA-Z0-9()@:%_\+.~#?&//=]{1,})(\?.*)?(\|\|)?"
    ).unwrap();
}

pub async fn handler(ctx: &Context, msg: &Message) {
    message_fixer(ctx, msg, &*RE, "https://ddinstagram.com", 6, (1, 8)).await;
}

pub fn enroll() -> Listener {
    let switch: bool = env::var("INSTA_SWITCH")
        .unwrap_or("true".to_string())
        .parse()
        .unwrap();

    Listener {
        name: "insta".to_string(),
        switch: switch.clone(),
    }
}
