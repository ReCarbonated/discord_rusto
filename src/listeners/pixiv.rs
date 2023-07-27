use crate::PixivClientHold;
use regex::Regex;
use serenity::client::Context;
use serenity::model::channel::Message;
use std::path::PathBuf;
use std::{env, fs};
use tokio::time::{Duration, sleep};

// use super::generic::message_fixer;
use super::Listener;
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
                println!("[pixiv][handler] Found a regex match: {}", artwork_id.as_str());
                let data = ctx.data.read().await;
                let pixiv_client = data
                    .get::<PixivClientHold>()
                    .expect("Expected Pixiv Client in TypeMap")
                    .clone();
                match pixiv_client
                    .download_image(artwork_id.as_str(), Option::None)
                    .await
                {
                    Ok(illust) => {
                        let mut paths = Vec::<PathBuf>::new();
                        for path in fs::read_dir(format!("./{}", artwork_id.as_str())).unwrap() {
                            match path {
                                Ok(path) => {
                                    let temp = path.path();
                                    paths.push(temp);
                                }
                                Err(_) => {
                                    println!("Something fucking died here")
                                }
                            }
                        }

                        let mut image_count = "".to_string();
                        if paths.len() > 4 {
                            image_count = format!(" - {} images", paths.len());
                        }
                        

                        let mut images = paths.iter().take(4).collect::<Vec<&PathBuf>>();
                        images.sort();
                        let mut images = images.iter();

                        // Build a message object to send to channel
                        let _res = msg
                            .channel_id
                            .send_message(&ctx.http, |m| {
                                // construct new iter for images because of embed format
                                m.add_embed(|e| {
                                    e.author(|a| a.name(illust.user_name))
                                        .title(format!("{}{}", illust.title, image_count));

                                    // Error handling on next value
                                    match images.next() {
                                        Some(image) => {
                                            let image = image.file_name().unwrap().to_str().unwrap();
                                            // println!("{}", format!("attachment://{}", image));
                                            e.attachment(format!("{}", image));
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
                                for image in images {
                                    let image = image.file_name().unwrap().to_str().unwrap();
                                    println!("{}", format!("attachment://{}", image));
                                    m.add_embed(|e| {
                                        e.attachment(format!("{}", image)).url(
                                            x.get(0).expect("Something actually here").as_str(),
                                        )
                                    });
                                }
                                // Append reply to message
                                m.reference_message((msg.channel_id, msg.id));
                                m.add_files(paths.iter().map(|e| e.as_path()));
                                m
                            })
                            .await;

                        // Now delete the files that you just downloaded
                        let _ = fs::remove_dir_all(format!("./{}", artwork_id.as_str()));

                        sleep(Duration::from_secs(5)).await;
                        let mut message = msg.clone();
                        match message.suppress_embeds(&ctx.http).await {
                            Ok(_) => {
                                println!("[pixiv][handler]: Removed embed");
                            }
                            Err(_) => {
                                println!("[pixiv][handler]: Failed to remove, no perms");
                            }
                        }
                    }
                    Err(err) => {
                        println!("Failed to download, {:?}", err)
                    }
                }
            }
            None => {
                // Didn't find the group somehow?, might not be a note or something
                println!("Didn't find a match with the regex, weird? {:?}", x);
            }
        },
        None => {
            // Didn't find a regex match
        }
    }
}

pub fn enroll() -> (String, Listener) {
    let switch: bool = env::var("PIXIV_SWITCH")
        .unwrap_or("true".to_string())
        .parse()
        .unwrap();

    (
        "pixiv".to_string(),
        Listener {
            name: "pixiv".to_string(),
            switch: switch.clone(),
        },
    )
}
