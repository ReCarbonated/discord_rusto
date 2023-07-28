// Stolen from github
// https://github.com/pxseu/zettai-ryouiki

// Looks to do a bunch of things that I need

use anyhow::{anyhow, bail, Context, Result};
use serenity::futures;
use tokio::fs;

use crate::types::pixiv::{Illust, Response};

const BASE_HOSTNAME: &str = "www.pixiv.net";
const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/115.0";

pub fn get_base_path(original_url: String) -> String {
    let mut base_url_split = original_url.split("/").collect::<Vec<&str>>();

    base_url_split.pop();
    base_url_split.clone().join("/")
}

pub fn get_ext(original_url: String) -> String {
    original_url
        .split(".")
        .collect::<Vec<&str>>()
        .pop()
        .unwrap()
        .to_string()
}

#[derive(Debug, Clone)]
pub struct Pixiv {
    client: reqwest::Client,
}

impl Default for Pixiv {
    fn default() -> Self {
        Self::new(None).unwrap()
    }
}

impl Pixiv {
    pub fn new(cookie: Option<String>) -> Result<Self> {
        use reqwest::header::{HeaderMap, HeaderValue};

        let mut headers = HeaderMap::new();

        headers.insert("user-agent", HeaderValue::from_str(USER_AGENT)?);
        headers.insert("authority", HeaderValue::from_str(BASE_HOSTNAME)?);
        headers.insert(
            "referer",
            HeaderValue::from_str(&format!("https://{BASE_HOSTNAME}/"))?,
        );

        if let Some(cookie) = cookie {
            headers.insert("cookie", HeaderValue::from_str(cookie.as_str())?);
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Pixiv { client })
    }

    pub async fn download_image(&self, illust_id: &str, author: Option<String>) -> Result<Illust> {
        println!("Prepping download of illust info");
        let illust = self.get_illust(illust_id.clone()).await?;
        println!("Finished download of illust info");

        let mut page_count = illust.page_count;
        if page_count > 4 {
            page_count = 4
        }

        // println!("{}", format!(
        //     "Fetching images of {illust_id} ({} pages)",
        //     page_count
        // ));

        let base_url = get_base_path(illust.urls.original.clone());

        let ext = get_ext(illust.urls.original.clone());

        let full_path = match author {
            Some(path) => format!("u_{path}/i_{illust_id}"),
            None => format!("{illust_id}"),
        };

        fs::create_dir_all(full_path.clone()).await?;

        match illust.illust_type {
            // map each page to a future that downloads the image
            0 => {
                let downloads = (0..page_count).into_iter().map(|page| {
                    self.fetch_and_save_image(
                        format!("{base_url}/{illust_id}_p{page}.{ext}"),
                        format!("{full_path}/p{page}.{ext}"),
                    )
                });

                for task in futures::future::join_all(downloads).await.into_iter() {
                    if let Err(e) = task {
                        println!("{}", format!("{e:?}"));
                    }
                }
            }

            _ => bail!("Unsupported illust type: {}, skipping", illust.illust_type),
        };

        // println!("{}", format!("Done! {full_path}"));

        Ok(illust)
    }

    async fn fetch_and_save_image(&self, url: String, path: String) -> Result<()> {
        if fs::metadata(&path).await.is_ok() {
            println!("{}", format!("{path} already exists, skipping"));
            return Ok(());
        }

        // println!("{}", format!("Downloading {path}"));

        let bytes = self.fetch_bytes(&url).await?;

        // println!("{}", format!("Saving {path}"));

        fs::write(path, bytes).await?;

        Ok(())
    }

    async fn get_illust(&self, illust_id: &str) -> Result<Illust> {
        let data = self
            .client
            .get(format!("https://www.pixiv.net/ajax/illust/{illust_id}"))
            .send()
            .await?
            .error_for_status()?
            .json::<Response<Illust>>()
            .await
            .with_context(|| anyhow!("Failed to get illust: {illust_id}"))?;

        Ok(data.body)
    }

    pub async fn get_illust_json(&self, illust_id: &str) -> Result<String> {
        let data = self
            .client
            .get(format!("https://www.pixiv.net/ajax/illust/{illust_id}"))
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
            .unwrap();

        Ok(data)
    }

    async fn fetch_bytes(&self, url: &str) -> Result<Vec<u8>> {
        let data = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(data.to_vec())
    }
}
