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
) {
    match re.captures(&msg.content) {
        Some(x) => {
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
                    // msg.reply(&ctx.http, &rebuilt_url).await.unwrap();

                    let _ = msg.channel_id.send_message(&ctx.http, |m| {m.content(&rebuilt_url)}).await;

                    if delete_embed {
                        sleep(Duration::from_secs(5)).await;
                        let mut message = msg.clone();
                        match message.suppress_embeds(&ctx.http).await {
                            Ok(_) => {
                                println!("[generic][handler]: Removed embed");
                            }
                            Err(_) => {
                                println!("[generic][handler]: Failed to remove, no perms");
                            }
                        }
                    }
                }
                None => {}
            }
        }
        None => {}
    }
}
