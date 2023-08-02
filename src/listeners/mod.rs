use std::collections::HashMap;

use serenity::client::Context;
use serenity::model::channel::Message;

pub(crate) mod exhentai;
pub(crate) mod generic;
pub(crate) mod instagram;
pub(crate) mod misskey;
pub(crate) mod pixiv;
pub(crate) mod tiktok;
pub(crate) mod twitter;
pub(crate) mod vt_tiktok;

pub struct Listener {
    pub name: String,
    pub switch: bool,
}

pub async fn check_parsers(ctx: &Context, msg: &Message, settings: &HashMap<u64, crate::types::Setting>) {
    // First get the settings info for the settings message 
    let setting = settings.get(msg.guild_id.unwrap().as_u64()).unwrap().listeners;

    if setting.insta {instagram::handler(&ctx, &msg).await;}
    if setting.tiktok {tiktok::handler(&ctx, &msg).await;}
    if setting.vt_tiktok {vt_tiktok::handler(&ctx, &msg).await;}
    if setting.exhentai {exhentai::handler(&ctx, &msg).await;}
    if setting.misskey {misskey::handler(&ctx, &msg).await;}
    if setting.pixiv {pixiv::handler(&ctx, &msg).await;}
    if setting.twitter {twitter::handler(&ctx, &msg).await;}
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
