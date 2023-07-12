use serenity::framework::standard::macros::{check, command, group, help, hook};
use serenity::framework::standard::{
    help_commands,
    Args,
    CommandGroup,
    CommandOptions,
    CommandResult,
    DispatchError,
    HelpOptions,
    Reason,
    StandardFramework,
};

async fn is_owner(ctx: &Context, msg: &Message) -> bool {
    let data = ctx.data.read().await;
    let owner: u64 = data.get::<Owner>().expect("Expected Owner in TypeHash");

    msg.author.id == owner
}

async fn is_editor(ctx: &Context, msg: &Message) -> bool {
    let data = ctx.data.read().await;
    let editors: HashSet<u64> = data.get::<Editors>().expect("Expected Editors in TypeHash");

    editors.contains(msg.author.id)
}


#[check]
#[name = "Owner"]
async fn owner_check(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    if msg.author.id != 7 {
        return Err(Reason::User("Lacked owner permission".to_string()));
    }

    Ok(())
}