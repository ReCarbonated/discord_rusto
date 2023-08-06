use crate::PixivClientHold;
use crate::helpers::pixiv;
use regex::Regex;
use serenity::client::Context;
use serenity::model::channel::Message;
use tokio::time::{sleep, Duration};

use lazy_static::lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(\|\|)?http(s)*:\/\/(www\.)?(mobile\.)?(pixiv.net)\b(\/\w{2})?(\/artworks\/[\d\/]*)(\|\|)?"
    ).unwrap();

    static ref RE2: Regex = Regex::new(
        r"(\|\|)?http(s)*:\/\/(www\.)?(mobile\.)?(pixiv.net)\b(\/\w{2})?(\/artworks\/)([\d\/]*)(\|\|)?"
    ).unwrap();
}

// pub async fn fallback_handler(ctx: &Context, msg: &Message) {
//     message_fixer(ctx, msg, &*RE, "https://www.ppxiv.net", 7, (1, 8), false).await;
// }

pub async fn handler(ctx: &Context, msg: &Message) {
    match RE2.captures(&msg.content) {
        Some(x) => match x.get(8) {
            Some(artwork_id) => {
                println!(
                    "[pixiv][handler] Found a regex match: {}",
                    artwork_id.as_str()
                );
                let pixiv_client = {
                    let data = ctx.data.read().await;
                    data
                        .get::<PixivClientHold>()
                        .expect("Expected Pixiv Client in TypeMap")
                        .clone()
                };
                match pixiv_client
                    .download_image_data(artwork_id.as_str())
                    .await
                {
                    Ok((images, illust)) => {

                        let mut image_count: String = "".to_string();
                        if illust.page_count > 1 {
                            image_count = format!(" - {} images", illust.page_count);
                        }
                        let ext = pixiv::get_ext(illust.urls.original.clone());

                        let filenames: Vec<String> = (0..images.len()).into_iter().map(|e| format!("p{e}.{ext}")).collect();

                        // Build zip iter to map name to image
                        let mut dataset = images.iter().zip(filenames.clone());

                        // Build a message object to send to channel
                        let _res = msg
                            .channel_id
                            .send_message(&ctx.http, |m| {
                                // construct new iter for images because of embed format
                                m.add_embed(|e| {
                                    e.author(|a| a.name(illust.user_name))
                                        .title(format!("{}{}", illust.title, image_count));

                                    // Error handling on next value
                                    match dataset.next() {
                                        Some((_, filename)) => {
                                            // println!("{}", format!("attachment://{}", image));
                                            e.attachment(format!("{}", filename));
                                        }
                                        _ => {
                                            println!("huh");
                                        }
                                    }

                                    // Assign URL because discord groups via url
                                    e.url(x.get(0).expect("Something actually here").as_str());

                                    e
                                });

                                // For any leftover images, append more embeds with same url as above
                                for (_, filename) in dataset {
                                    println!("{}", format!("attachment://{}", filename));
                                    m.add_embed(|e| {
                                        e.attachment(format!("{}", filename)).url(
                                            x.get(0).expect("Something actually here").as_str(),
                                        )
                                    });
                                }
                                // Append reply to message
                                m.reference_message((msg.channel_id, msg.id));
                                m.allowed_mentions(|am| {
                                    am.replied_user(false);
                                    am
                                });
                                for (image, filename) in images.iter().zip(filenames) {
                                    m.add_file((image.as_slice(), filename.as_str()));
                                }
                                m
                            }
                        ).await;

                        sleep(Duration::from_secs(5)).await;
                        let mut message = msg.clone();
                        match message.suppress_embeds(&ctx.http).await {
                            Ok(_) => {
                                println!("[pixiv][handler]: Removed embed");
                            }
                            Err(_) => {
                                eprintln!("[pixiv][handler]: Failed to remove, no perms");
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Failed to download, {:?}", err);
                    }
                }
            }
            None => {
                // Didn't find the group somehow?, might not be a note or something
                eprintln!("Didn't find a match with the regex, weird? {:?}", x);
            }
        },
        None => {
            // Didn't find a regex match
        }
    }
}
