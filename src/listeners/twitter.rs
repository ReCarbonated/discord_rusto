use super::generic::message_fixer;
use lazy_static::lazy_static;
use regex::Regex;
use serenity::client::Context;
use serenity::model::channel::Message;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(\|\|)?http(s)*:\/\/(www\.)?(mobile\.)?(twitter\.com|x\.com)\b([-a-zA-Z0-9()@:%_\+.~#?&//=]+\/status\/[-a-zA-Z0-9()@:%_\+.~#&//=]+){1,}(\|\|)?",
    ).unwrap();
}

pub async fn handler(ctx: &Context, msg: &Message) {
    message_fixer(ctx, msg, &*RE, "https://vxtwitter.com", 6, (1, 7), true, true, "twitter").await;
}
