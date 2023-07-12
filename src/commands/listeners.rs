use serenity::framework::standard::macros::{check, command, group, help, hook};
use serenity::framework::standard::{
    help_commands,
    Args,
    CommandGroup,
    CommandOptions,
    CommandResult,
};
use serenity::prelude::Context;
use serenity::model::prelude::Message;

#[group]
#[commands(toggle)]
struct ListenerCommand;


#[command]
#[sub_commands(status)]
async fn toggle(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        status(&ctx, &msg).await;
    } else {
        let listener = args.single::<String>().expect("Expected string here");
        if ((is_owner(ctx, msg) || is_editor(ctx, msg))){
            let data = ctx.data.read().await;
            let listners: HashMap<String, Listener> = data
                .get_mut::<MessageListener>()
                .expect("Expected MessageListener in TypeHash");

            listeners.get(listener);
        } else {
            println!{"Someone tried to call the toggle function"}
        }
    }

    Ok(())
}


#[command]
async fn status(ctx: &Context, msg: &Message){
    let listners = ctx.data.read().await;
    let listners: &Vec<Listener> = listners.get::<MessageListener>().expect("Expected MessageListener in TypeHash");
    let _ = msg.channel_id.send_message(&ctx.http, |m|
        {
            m.add_embed(|e| {
                for listener in listners {
                    e.field(listener.name.to_string(), listener.switch.to_string(), true);
                }
                e
            })
        }
    ).await;
}