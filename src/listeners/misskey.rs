use serde::Deserialize;
use serenity::client::Context;
use serenity::model::channel::Message;
use std::collections::HashMap;

use super::Handler;

#[derive(Deserialize)]
struct Note {
    files: Vec<File>,
    user: User,
    text: String,
    #[serde(rename(deserialize = "createdAt"))]
    created_at: String,
}

#[derive(Deserialize, Clone)]
struct User {
    name: String,
    username: String,
    #[serde(rename(deserialize = "avatarUrl"))]
    avatar_url: String,
}

#[derive(Deserialize, Clone)]
struct File {
    url: String,
    #[serde(rename(deserialize = "type"))]
    file_type: String,
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
                                // Build iter of videos and images
                                let mut videos = parsed
                                    .files
                                    .iter()
                                    .filter(|f| f.file_type.contains("video"));
                                let mut images = parsed
                                    .files
                                    .iter()
                                    .filter(|f| f.file_type.contains("image"));

                                // Build peekables and check if contains any value
                                let contains_video = videos.by_ref().peekable().peek().is_some();
                                let contains_image = images.by_ref().peekable().peek().is_some();

                                // Shared res to prevent duplication
                                let mut _res;

                                // If there is a value in videos list, assume it's all videos
                                if contains_video {
                                    for file in videos {
                                        // Just reply to poster with links to videos
                                        _res = &msg.reply(&ctx.http, file.url.clone()).await;
                                    }

                                // If there's a value in image list, assume it's all images
                                } else if contains_image {
                                    // Build a message object to send to channel
                                    let _res = msg
                                        .channel_id
                                        .send_message(&ctx.http, |m| {
                                            // construct new iter for images because of embed format
                                            m.add_embed(|e| {
                                                e.author(|a| {
                                                    a.icon_url(&parsed.user.avatar_url)
                                                        .name(&parsed.user.name)
                                                        .url(format!(
                                                            "https://misskey.io/@{}",
                                                            parsed.user.username
                                                        ))
                                                })
                                                .description(&parsed.text)
                                                .timestamp(parsed.created_at.as_str());

                                                // Error handling on next value
                                                if let Some(image) = images.next() {
                                                    e.image(image.url.clone());
                                                }

                                                // Assign URL because discord groups via url
                                                e.url(format!(
                                                    "https://misskey.io/notes/{}",
                                                    note_id.as_str()
                                                ));

                                                e
                                            });

                                            // For any leftover images, append more embeds with same url as above
                                            for file in images {
                                                m.add_embed(|e| {
                                                    e.image(file.url.clone()).url(format!(
                                                        "https://misskey.io/notes/{}",
                                                        note_id.as_str()
                                                    ))
                                                });
                                            }
                                            // Append reply to message
                                            m.reference_message(msg);
                                            m
                                        })
                                        .await;
                                }
                            }
                            Err(_) => {
                                println!("Error trying to read response")
                            }
                        },
                        Err(_) => println!("Error trying to access api"),
                    }
                }
                None => {
                    // Didn't find the group somehow?, might not be a note or something
                }
            },
            None => {
                // Didn't find a regex max
            }
        }
    }
}
