use regex::Regex;
use serenity::client::Context;
use serenity::model::channel::Message;
use tokio::time::{sleep, Duration};

use crate::DbPool;

// pub struct Listener<C>{
//     pub caller: C,
//     pub name: String
// }

// impl<C> Listener<C>
// where 
//     C: Fn(&Context, &Message) -> dyn Future<Output = ()>
// {
//     pub fn new(func: C, name: String) -> Self {
//         Listener { caller: func, name: name }
//     }

//     pub async fn parse(&self, ctx: &Context, msg: &Message) {
//         (self.caller)(ctx, msg);
//     }
// }


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
            println!("[{}][message_fixer][{}] There was a match with url: {}", caller, msg.id.to_string(), x.get(0).unwrap().as_str());
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
                            let res = msg.reply(&ctx.http, &rebuilt_url).await;
                            match res {
                                Ok(sent_message) => {
                                    let pool = {
                                        let data = ctx.data.read().await;
                                        data.get::<DbPool>()
                                        .expect("Expected WebClient in TypeMap")
                                        .clone()
                                    };
    
                                    crate::helpers::sent_message_to_db(sent_message.id.as_u64(), msg.id.as_u64(), &pool).await;
                                },
                                Err(_) => {},
                            }

                            // let _ = msg.channel_id.send_message(&ctx.http, |m| {m.content(&rebuilt_url)}).await;

                            if delete_embed {
                                sleep(Duration::from_secs(3)).await;
                                let mut message = msg.clone();
                                match message.suppress_embeds(&ctx.http).await {
                                    Ok(_) => {
                                        println!("[{}][handler][{}][{}]: Removed embed", caller, message.id.to_string(), "true");
                                    }
                                    Err(_) => {
                                        eprintln!("[{}][handler][{}][{}]: Failed to remove, no perms", caller, message.id.to_string(), "true");
                                    }
                                }
                            }
                        }
                        false => {
                            match check_if_embeded(ctx, msg.clone(), 3).await {
                                true => {}
                                false => {
                                    let res = msg.reply(&ctx.http, &rebuilt_url).await;
                                    match res {
                                        Ok(sent_message) => {
                                            let pool = {
                                                let data = ctx.data.read().await;
                                                data.get::<DbPool>()
                                                .expect("Expected WebClient in TypeMap")
                                                .clone()
                                            };
            
                                            crate::helpers::sent_message_to_db(sent_message.id.as_u64(), msg.id.as_u64(), &pool).await;
                                        },
                                        Err(_) => {},
                                    }

                                    // let _ = msg.channel_id.send_message(&ctx.http, |m| {m.content(&rebuilt_url)}).await;

                                    if delete_embed {
                                        sleep(Duration::from_secs(2)).await;
                                        let mut message = msg.clone();
                                        match message.suppress_embeds(&ctx.http).await {
                                            Ok(_) => {
                                                println!("[{}][handler][{}][{}]: Removed embed", caller, message.id.to_string(), "false");
                                            }
                                            Err(_) => {
                                                eprintln!("[{}][handler][{}][{}]: Failed to remove, no perms", caller, message.id.to_string(), "false");
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
