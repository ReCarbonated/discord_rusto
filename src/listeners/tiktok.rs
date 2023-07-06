use serenity::model::channel::Message;
use serenity::client::Context;

use super::Handler;

impl Handler {
    pub async fn tiktok_handler(&self, ctx: &Context, msg: &Message) {
        self.message_fixer(ctx, msg, &self.tiktok_regex, "https://tiktxk.com", 6, (1, 8))
            .await;
    }
}