use std::collections::HashMap;

use serenity::client::Context;
use serenity::model::channel::Message;

pub(crate) mod generic;
pub(crate) mod instagram;
pub(crate) mod misskey;
pub(crate) mod pixiv;
pub(crate) mod tiktok;
pub(crate) mod twitter;

pub struct Listener {
    pub name: String,
    pub switch: bool,
}

pub async fn check_parsers(ctx: &Context, msg: &Message, listeners: &HashMap<String, Listener>) {
    for (_, listener) in listeners.iter() {
        if listener.switch {
            match listener.name.as_str() {
                "insta" => {
                    instagram::handler(&ctx, &msg).await;
                }
                "tiktok" => {
                    tiktok::handler(&ctx, &msg).await;
                }
                "pixiv" => {
                    pixiv::handler(&ctx, &msg).await;
                }
                "misskey" => {
                    misskey::handler(&ctx, &msg).await;
                }
                "twitter" => {
                    twitter::handler(&ctx, &msg).await;
                }
                _ => {}
            }
        }
    }
}

pub struct Handler;
