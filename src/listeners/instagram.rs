use lazy_static::lazy_static;
use regex::Regex;
use serenity::client::Context;
use serenity::model::channel::Message;

use super::generic::message_fixer;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(\|\|)?http(s)*:\/\/(www\.)?(mobile\.)?(instagram.com)\b(\/(?:reel|p)\/[-a-zA-Z0-9()@:%_\+.~#?&=]{1,}\/)(\?.*)?(\|\|)?"
    ).unwrap();
}

pub async fn handler(ctx: &Context, msg: &Message) {
    message_fixer(ctx, msg, &*RE, "https://ddinstagram.com", 6, (1, 8), true, true, "instagram").await;
}
