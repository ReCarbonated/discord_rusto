use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::timestamp::Timestamp;
use serenity::model::prelude::{Message, UserId};
use serenity::model::user::User;
use serenity::prelude::Context;
use sqlx::types::chrono::Utc;

use std::collections::HashMap;

use crate::commands::{is_editor, is_owner};


#[group]
#[commands(timer)]
struct TimerCommand;

struct Timer {
    time: i64,
    owner: u64,
    topic: String
}

impl Timer {
    pub fn new(discord_time: Timestamp, user: User, topic: String) -> Self {
        Timer {
            time: discord_time.timestamp(),
            owner: *user.id.as_u64(),
            topic: topic
        }
    }

    pub fn is_ready(&self) -> bool {
        Utc::now().timestamp() > self.time
    }

    pub fn to_userid(&self) -> UserId {
        UserId::from(self.owner)
    }
}

#[command]
async fn timer(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    Ok(())
}