use regex::Regex;
use serenity::client::Context;
use serenity::model::channel::Message;
use tokio::time::{sleep, Duration};

pub async fn message_fixer(
    ctx: &Context,
    msg: &Message,
    re: &Regex,
    url_fix: &str,
    group: usize,
    spoilers: (usize, usize),
    delete_embed: bool,
    ignore_check: bool,
    caller: &str
) {
    match re.captures(&msg.content) {
        Some(x) => {
            println!("[{}][message_fixer] There was a match with url: {}", caller, x.get(0).unwrap().as_str());
            match x.get(group) {
                Some(post_fix) => {
                    // Rebuild message here, only getting the first value because I don't care anymore
                    let mut spoiler_wrap = "";
                    if x.get(spoilers.0).is_some() && x.get(spoilers.1).is_some() {
                        spoiler_wrap = "||";
                    }

                    let rebuilt_url = format!(
                        "{}{}{}{}",
                        spoiler_wrap,
                        url_fix,
                        post_fix.as_str(),
                        spoiler_wrap
                    );
                    match ignore_check {
                        true => {
                            msg.reply(&ctx.http, &rebuilt_url).await.unwrap();

                            // let _ = msg.channel_id.send_message(&ctx.http, |m| {m.content(&rebuilt_url)}).await;

                            if delete_embed {
                                sleep(Duration::from_secs(2)).await;
                                let mut message = msg.clone();
                                match message.suppress_embeds(ctx).await {
                                    Ok(_) => {
                                        println!("[{}][handler]: Removed embed", caller);
                                    }
                                    Err(_) => {
                                        eprintln!("[{}][handler]: Failed to remove, no perms", caller);
                                    }
                                }
                            }
                        }
                        false => {
                            match check_if_embeded(ctx, msg.clone(), 3).await {
                                true => {}
                                false => {
                                    msg.reply(&ctx.http, &rebuilt_url).await.unwrap();

                                    // let _ = msg.channel_id.send_message(&ctx.http, |m| {m.content(&rebuilt_url)}).await;

                                    if delete_embed {
                                        sleep(Duration::from_secs(2)).await;
                                        let mut message = msg.clone();
                                        match message.suppress_embeds(ctx).await {
                                            Ok(_) => {
                                                println!("[{}][handler]: Removed embed", caller);
                                            }
                                            Err(_) => {
                                                eprintln!("[{}][handler]: Failed to remove, no perms", caller);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                None => {
                    eprintln!("[{}][message_fixer] Failed to get group at loc: {}", caller, group);
                }
            }
        }
        None => {}
    }
}

pub async fn check_if_embeded(ctx: &Context, msg: Message, seconds: u32) -> bool {
    let message_id = msg.id.0;
    let channel_id = msg.channel_id.0;
    for _ in 0..seconds {
        if !ctx
            .http
            .get_message(channel_id, message_id)
            .await
            .unwrap()
            .embeds
            .is_empty()
        {
            return true;
        }
        sleep(Duration::from_secs(1)).await;
    }
    return false;
}
