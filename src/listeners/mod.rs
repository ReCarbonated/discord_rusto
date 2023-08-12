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

pub async fn check_parsers(ctx: &Context, msg: &Message, settings: &HashMap<u64, crate::types::Setting>) {
    // First get the settings info for the settings message 
    if let Some(guild_id) = msg.guild_id {
        let setting = settings.get(guild_id.as_u64()).unwrap().listeners;

        if setting.insta {instagram::handler(&ctx, &msg).await;}
        if setting.tiktok {tiktok::handler(&ctx, &msg).await;}
        if setting.vt_tiktok {vt_tiktok::handler(&ctx, &msg).await;}
        if setting.exhentai {exhentai::handler(&ctx, &msg).await;}
        if setting.misskey {misskey::handler(&ctx, &msg).await;}
        if setting.pixiv {pixiv::handler(&ctx, &msg).await;}
        if setting.twitter {twitter::handler(&ctx, &msg).await;}
    } else {
        eprintln!("[listeners][check_parsers] Failed to unwrap the message's guild_id");
    }

}

pub struct Handler;
