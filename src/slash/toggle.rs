use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::{
            application_command::{CommandDataOption, CommandDataOptionValue},
            command::CommandOptionType,
            GuildId,
        },
        user::User,
    }, prelude::Context,
};

use crate::{SettingsMap, types::Setting, DbPool};

// pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
//     command
//         .name("toggle")
//         .description("Toggles the bot to fix links or not, Admin required")
//         .create_option(|opt| {
//             opt.name("website")
//                 .description("Website to be toggled")
//                 .required(true)
//                 .kind(CommandOptionType::String)
//                 .add_string_choice("twitter", "twitter")
//                 .add_string_choice("misskey", "misskey")
//                 .add_string_choice("instagram", "insta")
//                 .add_string_choice("pixiv", "pixiv")
//                 .add_string_choice("tiktok", "tiktok")
//                 .add_string_choice("vt_tiktok", "vt_tiktok")
//         }
//     )
// }

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("listeners")
        .description("listener root for the bot")
        .create_option(|opt| {
            opt
                .name("status")
                .description("Prints the status of the listeners")
                .kind(CommandOptionType::SubCommand)
        })
        .create_option(|opt| {
            opt
                .name("toggle")
                .description("Toggle the specified listener")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|subopt| {
                    subopt.name("website")
                        .description("Website to be toggled")
                        .required(true)
                        .kind(CommandOptionType::String)
                        .add_string_choice("twitter", "twitter")
                        .add_string_choice("misskey", "misskey")
                        .add_string_choice("instagram", "insta")
                        .add_string_choice("pixiv", "pixiv")
                        .add_string_choice("tiktok", "tiktok")
                        .add_string_choice("vt_tiktok", "vt_tiktok")
                })
        })
}

pub async fn run(options: &[CommandDataOption], guild_id: &GuildId, user: &User, ctx: &Context) -> String {
    if let Some(command_data_option) = options.get(0) {
        match command_data_option.name.as_str() {
            "status" => {
                toggle_status(guild_id, ctx).await
            },
            "toggle" => {
                let command_value = command_data_option.options.get(0).unwrap().resolved.as_ref().unwrap();
                if let CommandDataOptionValue::String(output) = command_value {
                    toggle_state(output, guild_id, user, ctx).await
                } else {
                    "Something blew up".to_string()
                }
            },
            _ => {"Heh".to_string()},
        }
    } else {
        "Expected something".to_string()
    }
}

async fn toggle_status(guild_id: &GuildId, ctx: &Context) -> String {
    let listener;  
    {
        let data = ctx.data.read().await;
        listener = data
            .get::<SettingsMap>()
            .expect("Expected MessageListener in TypeHash")
            .get(guild_id.as_u64())
            .unwrap()
            .listeners;
    }
    let mut storage = Vec::new();
    storage.push(format!("vt_tiktok: {}", listener.vt_tiktok.to_string()));
    storage.push(format!("tiktok: {}", listener.tiktok.to_string()));
    storage.push(format!("exhentai: {}", listener.exhentai.to_string()));
    storage.push(format!("twitter: {}", listener.twitter.to_string()));
    storage.push(format!("pixiv: {}", listener.pixiv.to_string()));
    storage.push(format!("misskey: {}", listener.misskey.to_string()));
    storage.push(format!("insta: {}", listener.insta.to_string()));

    storage.join("\n")
}

async fn toggle_state(value: &str, guild_id: &GuildId, user: &User, ctx: &Context) -> String {
    let mut is_change = false;
    let is_ok = {
        let data = ctx.data.read().await;
        data
            .get::<SettingsMap>()
            .expect("Expected MessageListener in TypeHash")
            .get(guild_id.as_u64())
            .unwrap()
            .can_edit(&ctx, user, guild_id).await
    };
    if is_ok {
        // Insert list of viable options for Listeners
        match value {
            "vt_tiktok" => {
                {
                    let mut data = ctx.data.write().await;
                    let mapping = data
                        .get_mut::<SettingsMap>()
                        .expect("Expected MessageListener in TypeHash");
                    let setting = mapping.entry(*guild_id.as_u64());
                    let entry = setting.or_insert(Setting::new(*guild_id.as_u64()));
                    entry.listeners.vt_tiktok = !entry.listeners.vt_tiktok;
                }
                is_change = true;
            }
            "tiktok" => {
                {
                    let mut data = ctx.data.write().await;
                    let mapping = data
                        .get_mut::<SettingsMap>()
                        .expect("Expected MessageListener in TypeHash");
                    let setting = mapping.entry(*guild_id.as_u64());
                    let entry = setting.or_insert(Setting::new(*guild_id.as_u64()));
                    entry.listeners.tiktok = !entry.listeners.tiktok;
                }
                is_change = true;
            }
            "misskey" => {
                {
                    let mut data = ctx.data.write().await;
                    let mapping = data
                        .get_mut::<SettingsMap>()
                        .expect("Expected MessageListener in TypeHash");
                    let setting = mapping.entry(*guild_id.as_u64());
                    let entry = setting.or_insert(Setting::new(*guild_id.as_u64()));
                    entry.listeners.misskey = !entry.listeners.misskey;
                }
                is_change = true;
            }
            "pixiv" => {
                {
                    let mut data = ctx.data.write().await;
                    let mapping = data
                        .get_mut::<SettingsMap>()
                        .expect("Expected MessageListener in TypeHash");
                    let setting = mapping.entry(*guild_id.as_u64());
                    let entry = setting.or_insert(Setting::new(*guild_id.as_u64()));
                    entry.listeners.pixiv = !entry.listeners.pixiv;
                }
                is_change = true;
            }
            "insta" => {
                {
                    let mut data = ctx.data.write().await;
                    let mapping = data
                        .get_mut::<SettingsMap>()
                        .expect("Expected MessageListener in TypeHash");
                    let setting = mapping.entry(*guild_id.as_u64());
                    let entry = setting.or_insert(Setting::new(*guild_id.as_u64()));
                    entry.listeners.insta = !entry.listeners.insta;
                }
                is_change = true;
            }
            "exhentai" => {
                {
                    let mut data = ctx.data.write().await;
                    let mapping = data
                        .get_mut::<SettingsMap>()
                        .expect("Expected MessageListener in TypeHash");
                    let setting = mapping.entry(*guild_id.as_u64());
                    let entry = setting.or_insert(Setting::new(*guild_id.as_u64()));
                    entry.listeners.exhentai = !entry.listeners.vt_tiktok;
                }
                is_change = true;
            }
            "twitter" => {
                {
                    let mut data = ctx.data.write().await;
                    let mapping = data
                        .get_mut::<SettingsMap>()
                        .expect("Expected MessageListener in TypeHash");
                    let setting = mapping.entry(*guild_id.as_u64());
                    let entry = setting.or_insert(Setting::new(*guild_id.as_u64()));
                    entry.listeners.twitter = !entry.listeners.twitter;
                }
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
                .get(guild_id.as_u64())
                .unwrap();
            let pool = data.get::<DbPool>().unwrap();
            println!("{:?}", setting);
            crate::types::settings::insert_guild_setting(*guild_id.as_u64(), setting, pool).await;
            format!("{} toggled", value)
        } else {
            format!("{} did not toggle", value)
        }
    } else {
        // Someone tried to run it without perms
        "You don't have permissions".to_string()
    }
}