use serenity::model::channel::Message;
use serenity::client::Context;

use super::Handler;

impl Handler {
    // Alias function to call message_fixer for twitter
    pub async fn twitter_handler(&self, ctx: &Context, msg: &Message) {
        self.message_fixer(
            ctx,
            msg,
            &self.twitter_regex,
            "https://vxtwitter.com",
            6,
            (1, 7),
        )
        .await;
    }
}
