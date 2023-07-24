use std::collections::HashMap;

use serenity::client::Context;
use serenity::model::channel::Message;

pub(crate) mod generic;
pub(crate) mod instagram;
pub(crate) mod misskey;
pub(crate) mod pixiv;
pub(crate) mod tiktok;
pub(crate) mod vt_tiktok;
pub(crate) mod twitter;
pub(crate) mod exhentai;

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
                    // pixiv::fallback_handler(&ctx, &msg).await;
                }
                "misskey" => {
                    misskey::handler(&ctx, &msg).await;
                }
                "twitter" => {
                    twitter::handler(&ctx, &msg).await;
                },
                "vt_tiktok" => {
                    vt_tiktok::handler(&ctx, &msg).await;
                },
                "exhentai" => {
                    exhentai::handler(&ctx, &msg).await;
                }
                _ => {}
            }
        }
    }
}


pub fn gen_handlers() -> HashMap<String, Listener> {
    let list_of_listeners: HashMap<String, Listener> = {
        let mut collect = Vec::new();
        collect.push(tiktok::enroll());
        collect.push(misskey::enroll());
        collect.push(twitter::enroll());
        collect.push(instagram::enroll());
        collect.push(pixiv::enroll());
        collect.push(vt_tiktok::enroll());
        collect.push(exhentai::enroll());
        collect
    }
    .into_iter()
    .collect();

    list_of_listeners
}

pub struct Handler;
