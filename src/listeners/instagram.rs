use serenity::model::channel::Message;
use serenity::client::Context;

use super::Handler;

impl Handler {
    // Alias function to call message_fixer for instagram
    pub async fn insta_handler(&self, ctx: &Context, msg: &Message) {
        self.message_fixer(ctx, msg, &self.insta_regex, "https://ddinstagram.com", 6, (1, 8))
            .await;
    }
}