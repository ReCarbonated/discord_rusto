use serde::Deserialize;
use serenity::client::Context;
use serenity::model::channel::Message;
use std::collections::HashMap;

use super::Handler;

#[derive(Deserialize)]
struct Note {
    files: Vec<File>,
}
#[derive(Deserialize, Clone)]
struct File {
    url: String,
}

impl Handler {
    pub async fn misskey_handler(&self, ctx: &Context, msg: &Message) {
        match self.misskey_regex.captures(&msg.content) {
            Some(x) => match x.get(6) {
                Some(note_id) => {
                    let mut json = HashMap::new();
                    json.insert("noteId", note_id.as_str());
                    match self
                        .client
                        .post("https://misskey.io/api/notes/show")
                        .json(&json)
                        .send()
                        .await
                    {
                        Ok(res) => match &res.json::<Note>().await {
                            Ok(parsed) => {
                                for file in &parsed.files {
                                    msg.reply(&ctx.http, file.url.clone())
                                        .await
                                        .unwrap();
                                }
                            }
                            Err(_) => {}
                        },
                        Err(_) => println!("Error trying to access api"),
                    }
                }
                None => {}
            },
            None => {}
        }
    }
}