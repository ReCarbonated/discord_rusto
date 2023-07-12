use regex::Regex;
use serenity::model::channel::Message;
use serenity::client::Context;
use super::{Handler, Listener};
use crate::MessageListener;

impl Handler {
    pub async fn print_status(&self, ctx: &Context, msg: &Message){
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
}


pub async fn message_fixer(ctx: &Context, msg: &Message, re: &Regex, url_fix: &str, group: usize, spoilers: (usize, usize)) {
    match re.captures(&msg.content) {
        Some(x) => {
            match x.get(group) {
                Some(post_fix) => {
                    // Rebuild message here, only getting the first value because I don't care anymore
                    let mut spoiler_wrap = "";
                    if x.get(spoilers.0).is_some() && x.get(spoilers.1).is_some() {
                        spoiler_wrap = "||";
                    } 
                        
                    let rebuilt_url = format!("{}{}{}{}", spoiler_wrap, url_fix, post_fix.as_str(), spoiler_wrap);
                    msg.reply(&ctx.http, &rebuilt_url).await.unwrap();
                }
                None => {}
            }
        }
        None => {}
    }
}