use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{
    Args,
    CommandResult,
};
use serenity::prelude::Context;
use serenity::model::prelude::Message;

use std::collections::HashMap;

use crate::MessageListener;
use crate::commands::{is_editor, is_owner};
use crate::listeners::Listener;

#[group]
#[commands(toggle)]
struct ListenerCommand;


#[command]
#[sub_commands(toggle_status)]
async fn toggle(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        return toggle_status(&ctx, &msg, args).await;
    } else {
        let input = args.single::<String>().expect("Expected string here");
        if is_owner(ctx, msg).await || is_editor(ctx, msg).await {
            let mut data = ctx.data.write().await;
            let listeners = data
                .get_mut::<MessageListener>()
                .expect("Expected MessageListener in TypeHash");

            match listeners.get_mut(&input) {
                Some(listener) => {
                    listener.switch = !listener.switch;
                },
                None => {}
            }
        } else {
            println!{"Someone tried to call the toggle function"}
        }
    }

    Ok(())
}


#[command]
#[aliases("status")]
#[description("Get status of listeners")]
async fn toggle_status(ctx: &Context, msg: &Message) -> CommandResult {
    let listners = ctx.data.read().await;
    let listners: &HashMap<String, Listener> = listners.get::<MessageListener>().expect("Expected MessageListener in TypeHash");
    let _ = msg.channel_id.send_message(&ctx.http, |m|
        {
            m.add_embed(|e| {
                for (_, listener) in listners {
                    e.field(listener.name.to_string(), listener.switch.to_string(), true);
                }
                e
            })
        }
    ).await;

    Ok(())
}