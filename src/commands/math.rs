use serenity::{
    prelude::Context,
    model::channel::Message,
    framework::standard::{Args, CommandResult},
};
use serenity::framework::standard::macros::{command, group};

#[group]
#[commands(math, math2)]
struct Math;

#[command]
#[description = "huh."]
async fn math(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let _first = args.single::<f64>()?;
    let _second = args.single::<f64>()?;

    // let res = first * second;
    println!("{}", msg.content);
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
#[description = "huh."]
async fn math2(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = args.single::<f64>()?;
    let second = args.single::<f64>()?;

    let res = first * second;
    println!("{}", res);
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}