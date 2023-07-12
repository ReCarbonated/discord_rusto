use std::collections::HashSet;

use serenity::framework::standard::macros::check;
use serenity::framework::standard::{
    Args,
    CommandOptions,
    Reason,
};

use serenity::prelude::Context;
use serenity::model::prelude::Message;

use crate::{Owner, Editors};

pub async fn is_owner(ctx: &Context, msg: &Message) -> bool {
    let data = ctx.data.read().await;
    let owner: u64 = *data.get::<Owner>().expect("Expected Owner in TypeHash");

    msg.author.id == owner
}

pub async fn is_editor(ctx: &Context, msg: &Message) -> bool {
    let data = ctx.data.read().await;
    let editors: &HashSet<u64> = data.get::<Editors>().expect("Expected Editors in TypeHash");

    editors.contains(msg.author.id.as_u64())
}


#[check]
#[name = "Owner"]
async fn owner_check(
    _ctx: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    if msg.author.id != 7 {
        return Err(Reason::User("Lacked owner permission".to_string()));
    }

    Ok(())
}