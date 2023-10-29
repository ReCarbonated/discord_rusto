use std::time::Duration;

use regex::Regex;
use serde::{Deserialize, Serialize};
use serenity::client::Context;
use serenity::model::channel::Message;

use lazy_static::lazy_static;
use tokio::time::sleep;

use crate::WebClient;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(\|\|)?http(s)*:\/\/(www\.)?(mobile\.)?(pixiv.net)\b(\/\w{2})?(\/artworks\/[\d\/]*)(\|\|)?"
    ).unwrap();

    static ref RE2: Regex = Regex::new(
        r"(\|\|)?http(s)*:\/\/(www\.)?(mobile\.)?(pixiv.net)\b(\/\w{2})?(\/artworks\/)([\d\/]*)(\|\|)?"
    ).unwrap();

    static ref SPOILERS: (usize, usize) = (1, 8);
}

#[derive(Serialize, Deserialize, Debug)]
struct PhixivAPI {
    image_proxy_urls: Vec<String>,
    title: String,
    author_name: String,
}

pub async fn handler(ctx: &Context, msg: &Message) {
    // This was the old handler for pixiv, instead go into the API and grab the info instead.
    // message_fixer(ctx, msg, &*RE, "https://www.ppxiv.net", 7, (1, 8), false).await;
    match RE2.captures(&msg.content) {
        Some(x) => {
            match x.get(8) {
                Some(gallery_id) => {
                    // Rebuild message here, only getting the first value because I don't care anymore
                    let mut spoiler_wrap = false;
                    if x.get(SPOILERS.0).is_some() && x.get(SPOILERS.1).is_some() {
                        spoiler_wrap = true;
                    }
                    if !spoiler_wrap {
                        let client = {
                            let data = ctx.data.read().await;
                            data.get::<WebClient>()
                                .expect("Expected WebClient in TypeMap")
                                .clone()
                        };
                        let output = client
                            .get(format!("https://www.phixiv.net/api/info?id={}", gallery_id.as_str()))
                            .send()
                            .await
                            .unwrap()
                            .json::<PhixivAPI>()
                            .await
                            .unwrap();

                        // Build Rich Embed
                        let mut images = output.image_proxy_urls.iter().take(4);
                        let _res = msg
                            .channel_id
                            .send_message(&ctx.http, |m| {
                                // construct new iter for images because of embed format
                                m.add_embed(|e| {
                                    e.author(|a| a.name(&output.author_name));

                                    e.image(images.next().unwrap());

                                    // Assign URL because discord groups via url
                                    e.url(&msg.content);

                                    e
                                });

                                // For any leftover images, append more embeds with same url as above
                                for image in images {
                                    m.add_embed(|e| e.image(image).url(&msg.content));
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
                            match message.suppress_embeds(&ctx.http).await {
                                Ok(_) => {
                                    println!("[pixiv][handler][{}]: Removed embed", message.id.to_string());
                                }
                                Err(_) => {
                                    eprintln!("[pixiv][handler][{}]: Failed to remove, no perms", message.id.to_string());
                                }
                            }
                    }
                    // Now get the information from the phixiv API
                }
                None => {
                    // Didn't find an ID for some reason
                }
            }
        }
        None => {
            // Regex match didn't fit url
        }
    }
}
