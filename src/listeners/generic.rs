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
                        msg.channel_id.say(&ctx.http, &rebuilt_url).await.unwrap();
                    }
                    None => {}
                }
            }
            None => {}
        }
    }
}