use serenity::model::channel::Message;
use serenity::client::Context;

use super::Handler;

impl Handler {
    // Alias function to call message_fixer for pixiv
    pub async fn pixiv_handler(&self, ctx: &Context, msg: &Message) {
        self.message_fixer(
            ctx,
            msg,
            &self.pixiv_regex,
            "https://www.ppxiv.net",
            7,
            (1, 8),
        )
        .await;
    }
}
