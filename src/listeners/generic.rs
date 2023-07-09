use regex::Regex;
use serenity::model::channel::Message;
use serenity::client::Context;
use super::Handler;

impl Handler {
    // Function to rip out specific stuff out of groups, currently only rips out 6th group from regex
    // Then subsitute's the string provided and concats and then spits
    pub async fn message_fixer(&self, ctx: &Context, msg: &Message, re: &Regex, url_fix: &str, group: usize, spoilers: (usize, usize)) {
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

    pub async fn print_status(&self, ctx: &Context, msg: &Message){
        let lock = self.switches.lock().await;
        let _ = msg.channel_id.send_message(&ctx.http, |m|
            {
                m.add_embed(|e| {
                    e
                    .field("twitter", lock.twitter_switch.to_string(), true)
                    .field("insta", lock.insta_switch.to_string(), true)
                    .field("tiktok", lock.tiktok_switch.to_string(), true)
                    .field("misskey", lock.misskey_switch.to_string(), true)
                    .field("pixiv", lock.pixiv_switch.to_string(), true)
                })
            }
        
        
        
        
        ).await;

    }
}