use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Number, Value}; // 1.0.69

#[derive(Serialize, Debug)]
pub struct APIPayload {
    method: String,
    gidlist: Vec<Vec<Value>>,
    namespace: u32,
}

impl APIPayload {
    pub fn new(gallery_id: u32, gallery_token: String) -> Self {
        APIPayload {
            method: "gdata".to_string(),
            gidlist: {
                let mut gidlist: Vec<Value> = Vec::new();
                gidlist.push(Value::Number(Number::from(gallery_id)));
                gidlist.push(Value::String(gallery_token));

                let mut nested = Vec::new();
                nested.push(gidlist);
                nested
            },
            namespace: 1,
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum Metatag {
    #[serde(alias = "mixed")]
    Mixed(String),
    #[serde(alias = "parody")]
    Parody(String),
    #[serde(alias = "male")]
    Male(String),
    #[serde(alias = "female")]
    Female(String),
    #[serde(alias = "other")]
    Other(String),
    #[serde(alias = "artist")]
    Artist(String),
    #[serde(alias = "character")]
    Character(String),
    #[serde(alias = "group")]
    Group(String),
    #[serde(alias = "language")]
    Language(String),
    #[serde(alias = "temp")]
    Temp(String),
    #[serde(alias = "reclass")]
    Reclass(String),
    #[serde(alias = "cosplayer")]
    Cosplayer(String),
}

#[derive(Deserialize, Debug)]
pub struct GalleryMetaDataList {
    #[serde(rename(deserialize = "gmetadata"))]
    pub items: Vec<GalleryMetaData>,
}

#[derive(Deserialize, Debug)]
pub struct GalleryMetaData {
    pub gid: u32,
    pub token: String,
    pub title: String,
    pub category: String,
    #[serde(rename(deserialize = "thumb"))]
    pub thumbnail: String,
    #[serde(rename(deserialize = "filecount"))]
    #[serde(deserialize_with = "from_str")]
    pub file_count: u32,
    pub uploader: String,
    #[serde(deserialize_with = "from_tag")]
    pub tags: Vec<Metatag>,
}

fn from_str<'de, T, D>(de: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    Ok(String::deserialize(de)?
        .parse()
        .map_err(serde::de::Error::custom)?)
}

fn from_tag<'de, D>(deserializer: D) -> Result<Vec<Metatag>, D::Error>
where
    D: Deserializer<'de>,
{
    let list_of_tags: Vec<&str> = Deserialize::deserialize(deserializer)?;
    Ok(list_of_tags
        .iter()
        .map(|e| {
            let (meta, tag) = e.split_once(":").expect("Some how fucked up splitting on a colon");

            serde_json::from_str::<Metatag>(
                format!("{{\"{}\": \"{}\"}}", meta.to_string(), tag.to_string()).as_str(),
            )
            .unwrap_or(serde_json::from_str::<Metatag>(
                format!("{{\"{}\": \"{}\"}}", "other", tag.to_string()).as_str(),
            ).unwrap())
        })
        .collect::<Vec<_>>())
}
