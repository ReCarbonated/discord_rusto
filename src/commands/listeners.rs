use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use crate::types::Setting;
use crate::{SettingsMap, DbPool};

#[group]
#[commands(toggle)]
struct ListenerCommand;

#[command]
#[sub_commands(toggle_status, toggle_admin)]
async fn toggle(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        return toggle_status(&ctx, &msg, args).await;
    } else {
        let input = args
            .single::<String>()
            .expect("Expected string here")
            .to_lowercase();
        // Get the listener for the guild
        let mut is_ok = false;
        let mut is_change = false;
        {
            let data = ctx.data.read().await;
            is_ok = data
                .get::<SettingsMap>()
                .expect("Expected MessageListener in TypeHash")
                .get(msg.guild_id.unwrap().as_u64())
                .unwrap()
                .can_edit(msg.author.id.as_u64());
        }

        if is_ok {
            // Insert list of viable options for Listeners
            match input.as_str() {
                "vt_tiktok" => {
                    {
                        let mut data = ctx.data.write().await;
                        let mapping = data
                            .get_mut::<SettingsMap>()
                            .expect("Expected MessageListener in TypeHash");
                        let setting = mapping.entry(*msg.guild_id.unwrap().as_u64());
                        let entry = setting.or_insert(Setting::new(*msg.guild_id.unwrap().as_u64()));
                        entry.listeners.vt_tiktok = !entry.listeners.vt_tiktok;
                    }
                    let _ = msg.reply(&ctx.http, '👍').await;
                    is_change = true;
                }
                "tiktok" => {
                    {
                        let mut data = ctx.data.write().await;
                        let mapping = data
                            .get_mut::<SettingsMap>()
                            .expect("Expected MessageListener in TypeHash");
                        let setting = mapping.entry(*msg.guild_id.unwrap().as_u64());
                        let entry = setting.or_insert(Setting::new(*msg.guild_id.unwrap().as_u64()));
                        entry.listeners.tiktok = !entry.listeners.tiktok;
                    }
                    let _ = msg.reply(&ctx.http, '👍').await;
                    is_change = true;
                }
                "misskey" => {
                    {
                        let mut data = ctx.data.write().await;
                        let mapping = data
                            .get_mut::<SettingsMap>()
                            .expect("Expected MessageListener in TypeHash");
                        let setting = mapping.entry(*msg.guild_id.unwrap().as_u64());
                        let entry = setting.or_insert(Setting::new(*msg.guild_id.unwrap().as_u64()));
                        entry.listeners.misskey = !entry.listeners.misskey;
                    }
                    let _ = msg.reply(&ctx.http, '👍').await;
                    is_change = true;
                }
                "pixiv" => {
                    {
                        let mut data = ctx.data.write().await;
                        let mapping = data
                            .get_mut::<SettingsMap>()
                            .expect("Expected MessageListener in TypeHash");
                        let setting = mapping.entry(*msg.guild_id.unwrap().as_u64());
                        let entry = setting.or_insert(Setting::new(*msg.guild_id.unwrap().as_u64()));
                        entry.listeners.pixiv = !entry.listeners.pixiv;
                    }
                    let _ = msg.reply(&ctx.http, '👍').await;
                    is_change = true;
                }
                "insta" => {
                    {
                        let mut data = ctx.data.write().await;
                        let mapping = data
                            .get_mut::<SettingsMap>()
                            .expect("Expected MessageListener in TypeHash");
                        let setting = mapping.entry(*msg.guild_id.unwrap().as_u64());
                        let entry = setting.or_insert(Setting::new(*msg.guild_id.unwrap().as_u64()));
                        entry.listeners.insta = !entry.listeners.insta;
                    }
                    let _ = msg.reply(&ctx.http, '👍').await;
                    is_change = true;
                }
                "exhentai" => {
                    {
                        let mut data = ctx.data.write().await;
                        let mapping = data
                            .get_mut::<SettingsMap>()
                            .expect("Expected MessageListener in TypeHash");
                        let setting = mapping.entry(*msg.guild_id.unwrap().as_u64());
                        let entry = setting.or_insert(Setting::new(*msg.guild_id.unwrap().as_u64()));
                        entry.listeners.exhentai = !entry.listeners.vt_tiktok;
                    }
                    let _ = msg.reply(&ctx.http, '👍').await;
                    is_change = true;
                }
                "twitter" => {
                    {
                        let mut data = ctx.data.write().await;
                        let mapping = data
                            .get_mut::<SettingsMap>()
                            .expect("Expected MessageListener in TypeHash");
                        let setting = mapping.entry(*msg.guild_id.unwrap().as_u64());
                        let entry = setting.or_insert(Setting::new(*msg.guild_id.unwrap().as_u64()));
                        entry.listeners.twitter = !entry.listeners.twitter;
                    }
                    let _ = msg.reply(&ctx.http, '👍').await;
                    is_change = true;
                }
                _ => {
                    println!("Didn't get a value to match to")
                }
            }
            if is_change {
                let data = ctx.data.read().await;
                let setting = data
                    .get::<SettingsMap>()
                    .expect("Expected MessageListener in TypeHash")
                    .get(msg.guild_id.unwrap().as_u64())
                    .unwrap();
                let pool = data.get::<DbPool>().unwrap();
                println!("{:?}", setting);
                crate::types::settings::insert_guild_setting(*msg.guild_id.unwrap().as_u64(), setting, pool).await;
            }
        } else {
            // Someone tried to run it without perms
        }
    }
    Ok(())
}

#[command]
#[aliases("status")]
#[description("Get status of listeners")]
async fn toggle_status(ctx: &Context, msg: &Message) -> CommandResult {
    let listener;
    {
        let data = ctx.data.read().await;
        listener = data
            .get::<SettingsMap>()
            .expect("Expected MessageListener in TypeHash")
            .get(msg.guild_id.unwrap().as_u64())
            .unwrap().listeners;
    }
    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.add_embed(|e| {
                // Now "iterate" over the struct to get all the values
                e.field("vt_tiktok", listener.vt_tiktok.to_string(), true);
                e.field("tiktok", listener.tiktok.to_string(), true);
                e.field("exhentai", listener.exhentai.to_string(), true);
                e.field("twitter", listener.twitter.to_string(), true);
                e.field("pixiv", listener.pixiv.to_string(), true);
                e.field("misskey", listener.misskey.to_string(), true);
                e.field("insta", listener.insta.to_string(), true);
                e
            })
        })
        .await;

    Ok(())
}

#[command]
#[aliases("admin")]
#[description("Add admin")]
async fn toggle_admin(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut user_ids: Vec<u64> = Vec::new();
    if !msg.mentions.is_empty() {
        user_ids.extend(msg.mentions.iter().map(|e| e.id.as_u64()));
    } else {
        for temp in args.iter::<u64>(){
            let temp_val = temp.unwrap_or(0);
            if temp_val != 0 {
                user_ids.push(temp_val);
            }
        }
    }

    let is_admin;
    {
        let data = ctx.data.read().await;
        is_admin = data
            .get::<SettingsMap>()
            .expect("Expected MessageListener in TypeHash")
            .get(msg.guild_id.unwrap().as_u64())
            .unwrap().can_edit(&msg.author.id.as_u64());
    }

    if is_admin {
        {
            let mut data = ctx.data.write().await;
            let mapping = data
                .get_mut::<SettingsMap>()
                .expect("Expected MessageListener in TypeHash");
            let setting = mapping.entry(*msg.guild_id.unwrap().as_u64());
            let entry = setting.or_insert(Setting::new(*msg.guild_id.unwrap().as_u64()));
            entry.admins.extend(user_ids);
        }
        {
            let data = ctx.data.read().await;
            let setting = data
                .get::<SettingsMap>()
                .expect("Expected MessageListener in TypeHash")
                .get(msg.guild_id.unwrap().as_u64())
                .unwrap();
            let pool = data.get::<DbPool>().unwrap();
            println!("{:?}", setting);
            crate::types::settings::insert_guild_setting(*msg.guild_id.unwrap().as_u64(), setting, pool).await;
        }
    } else {
        // No perms
        println!("No Perms");
    }
    

    Ok(())
}