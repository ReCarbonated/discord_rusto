use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Deserializer};
use serenity::client::Context;
use serenity::model::channel::Message;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

use crate::WebClient;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(\|\|)?http(s)*:\/\/(www\.)?(mobile\.)?(misskey.io/notes/)\b([-a-zA-Z0-9()@:%_\+.~#?&//=]{1,})(\?.*)?(\|\|)?"
    ).unwrap();
}

#[derive(Deserialize)]
struct Note {
    files: Vec<File>,
    user: User,
    #[serde(deserialize_with = "parse_null")]
    text: String,
    #[serde(rename(deserialize = "createdAt"))]
    created_at: String,
}

fn parse_null<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or("".to_string()))
}

#[derive(Deserialize, Clone)]
struct User {
    name: String,
    username: String,
    #[serde(rename(deserialize = "avatarUrl"))]
    avatar_url: String,
}

#[derive(Deserialize)]
struct File {
    url: String,
    #[serde(rename(deserialize = "type"))]
    file_type: String,
    // #[serde(rename(deserialize = "isSensitive"))]
    // is_nsfw: bool,
}

pub async fn handler(ctx: &Context, msg: &Message) {
    match RE.captures(&msg.content) {
        Some(x) => {
            match x.get(6) {
                Some(note_id) => {
                    let mut json = HashMap::new();
                    json.insert("noteId", note_id.as_str());
                    let client = {
                        let data = ctx.data.read().await;
                        data.get::<WebClient>().expect("Expected WebClient from TypeMap").clone()
                    };

                    match client
                    .post("https://misskey.io/api/notes/show")
                    .json(&json)
                    .send()
                    .await
                {
                    Ok(res) => match &res.json::<Note>().await {
                        Ok(parsed) => {
                            // Build iter of videos and images
                            let videos = parsed
                                .files
                                .iter()
                                .filter(|f| f.file_type.contains("video"))
                                .collect::<Vec<_>>();
                            let images_list = parsed
                                .files
                                .iter()
                                .filter(|f| f.file_type.contains("image"))
                                .collect::<Vec<_>>();

                            let mut images = images_list.iter();

                            // Build peekables and check if contains any value
                            let contains_video = !videos.is_empty();
                            let contains_image = !images_list.is_empty();

                            // Shared res to prevent duplication
                            let mut _res;

                            // If there is a value in videos list, assume it's all videos
                            if contains_video {
                                println!("[misskey][handler]: Found Videos");
                                for file in videos {
                                    // Just reply to poster with links to videos
                                    _res = &msg.reply(&ctx.http, file.url.clone()).await;
                                }

                            // If there's a value in image list, assume it's all images
                            } else if contains_image {
                                println!("[misskey][handler]: Found Images");

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
                                            match images.next() {
                                                Some(image) => {
                                                    e.image(image.url.clone());
                                                }
                                                _ => {
                                                    println!("huh");
                                                }
                                            }

                                            // Assign URL because discord groups via url
                                            e.url(format!(
                                                "https://misskey.io/notes/{}",
                                                note_id.as_str()
                                            ));

                                            e
                                        });

                                        // For any leftover images, append more embeds with same url as above
                                        for image in images {
                                            m.add_embed(|e| {
                                                e.image(image.url.clone()).url(format!(
                                                    "https://misskey.io/notes/{}",
                                                    note_id.as_str()
                                                ))
                                            });
                                        }
                                        // Append reply to message
                                        m.reference_message((msg.channel_id, msg.id));
                                        m.allowed_mentions(|am| {
                                            am.replied_user(false);
                                            am
                                        });
                                        m
                                    })
                                    .await;

                                sleep(Duration::from_secs(5)).await;
                                let mut message = msg.clone();
                                match message.suppress_embeds(ctx).await {
                                    Ok(_) => {
                                        println!("[misskey][handler]: Removed embed");
                                    }
                                    Err(_) => {
                                        eprintln!("[misskey][handler]: Failed to remove, no perms");
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("[misskey][handler]: Error trying to read response: {}", err)
                        }
                    },
                    Err(err) => eprintln!("[misskey][handler]: Error trying to access api with id {} and with: {}", note_id.as_str(), err),
                }
                }
                None => {
                    // Didn't find the group somehow?, might not be a note or something
                    eprintln!("Didn't find a match with the regex, weird? {:?}", x);
                }
            }
        }
        None => {
            // Didn't find a regex match
        }
    }
}
