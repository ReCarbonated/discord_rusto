use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use serenity::client::Context;
use serenity::model::channel::Message;
use crate::{
    types::{APIPayload, GalleryMetaDataList, Metatag},
    WebClient,
};

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(\|\|)?https?:\/\/(?:exhentai.org|e-hentai.org(?:\/lofi)?)\/(?:g|mpv)\/(\d+)\/([0-9a-f]{10})\/?(\|\|)?"
    ).unwrap();
}

pub async fn handler(ctx: &Context, msg: &Message) {
    match RE.captures(&msg.content) {
        Some(x) => match x.get(2) {
            Some(gallery_id) => match x.get(3) {
                Some(gallery_token) => {
                    if msg.channel(&ctx.http).await.unwrap().is_nsfw() {
                        let json_payload = APIPayload::new(
                            gallery_id.as_str().parse::<u32>().unwrap(),
                            gallery_token.as_str().to_string(),
                        );

                        let api_resp = send_payload(ctx, json_payload).await.unwrap();
                        let gallery_data = api_resp.items.first().unwrap();
                        // The lamest way to generate these pools
                        let mut mixed_tags = Vec::new();
                        let mut parody_tags = Vec::new();
                        let mut male_tags = Vec::new();
                        let mut female_tags = Vec::new();
                        let mut other_tags = Vec::new();
                        let mut artist_tags = Vec::new();
                        let mut character_tags = Vec::new();
                        let mut group_tags = Vec::new();
                        let mut language_tags = Vec::new();
                        let mut temp_tags = Vec::new();
                        let mut cosplayer_tags = Vec::new();
                        let mut reclass_tags = Vec::new();

                        gallery_data.tags.iter().for_each(|t| match t {
                            Metatag::Artist(token) => artist_tags.push(token.clone()),
                            Metatag::Character(token) => character_tags.push(token.clone()),
                            Metatag::Female(token) => female_tags.push(token.clone()),
                            Metatag::Group(token) => group_tags.push(token.clone()),
                            Metatag::Male(token) => male_tags.push(token.clone()),
                            Metatag::Mixed(token) => mixed_tags.push(token.clone()),
                            Metatag::Other(token) => other_tags.push(token.clone()),
                            Metatag::Parody(token) => parody_tags.push(token.clone()),
                            Metatag::Language(token) => language_tags.push(token.clone()),
                            Metatag::Temp(token) => temp_tags.push(token.clone()),
                            Metatag::Reclass(token) => reclass_tags.push(token.clone()),
                            Metatag::Cosplayer(token) => cosplayer_tags.push(token.clone()),
                        });

                        // Build embed now

                        let _res = msg
                            .channel_id
                            .send_message(&ctx.http, |m| {
                                m.add_embed(|e| {
                                    e.image(gallery_data.thumbnail.clone())
                                        .title(html_escape::decode_html_entities(gallery_data.title.as_str()))
                                        .field("Category", gallery_data.category.clone(), true)
                                        .url(x.get(0).unwrap().as_str().to_string());
                                    if !language_tags.is_empty() {
                                        e.field("Language", language_tags.join(", "), true);
                                    }
                                    if !group_tags.is_empty() {
                                        e.field("Group", group_tags.join(", "), true);
                                    }
                                    if !artist_tags.is_empty() {
                                        e.field("Artist", artist_tags.join(", "), true);
                                    }
                                    if !cosplayer_tags.is_empty() {
                                        e.field("Cosplay", cosplayer_tags.join(", "), true);
                                    }
                                    if !parody_tags.is_empty() {
                                        e.field("Parody", parody_tags.join(", "), true);
                                    }
                                    if !character_tags.is_empty() {
                                        e.field("Character", character_tags.join(", "), true);
                                    }
                                    if !female_tags.is_empty() {
                                        e.field("Female", female_tags.join(", "), true);
                                    }
                                    if !male_tags.is_empty() {
                                        e.field("Male", male_tags.join(", "), true);
                                    }
                                    if !mixed_tags.is_empty() {
                                        e.field("Mixed", mixed_tags.join(", "), true);
                                    }
                                    if !other_tags.is_empty() {
                                        e.field("Other", other_tags.join(", "), true);
                                    }

                                    e
                                });

                                m.reference_message((msg.channel_id, msg.id));
                                m.allowed_mentions(|am| {
                                    am.replied_user(false);
                                    am
                                });
                                m
                            })
                            .await;
                    }
                }
                None => {
                    println!("Didn't find gallery token");
                }
            },
            None => {
                println!("Didn't find gallery_id");
            }
        },
        None => {
            // Didn't find a regex match
        }
    }
}

async fn send_payload(ctx: &Context, payload: APIPayload) -> Result<GalleryMetaDataList> {
    // println!("{}", serde_json::to_string(&payload).unwrap());
    let client = {
        let data = ctx.data.read().await;
        data.get::<WebClient>()
        .expect("Expected WebClient in TypeMap")
        .clone()
    };
    let output = client
        .post("https://api.e-hentai.org/api.php")
        .json(&payload)
        .send()
        .await?
        .json::<GalleryMetaDataList>()
        .await?;

    Ok(output)
}
