use serde::{Deserialize, Serialize, Deserializer};
use serde_json::{Value, Number}; // 1.0.69

#[derive(Serialize)]
struct APIPayload {
    method: String,
    gidlist: Vec<Vec<Value>>,
    namespace: u32,
}

impl APIPayload {
    fn new(gallery_id: u32, gallery_token: String) -> Self{
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
            namespace: 1
        }
    }
}


#[derive(Deserialize, Debug)]
enum Metatag {
    Unlisted(String),
    #[serde(alias="parody")]
    Parody(String),
    #[serde(alias="male")]
    Male(String),
    #[serde(alias="female")]
    Female(String),
    #[serde(alias="other")]
    Other(String),
    #[serde(alias="artist")]
    Artist(String),
    #[serde(alias="character")]
    Character(String),
    #[serde(alias="group")]
    Group(String)
}

#[derive(Deserialize, Debug)]
struct GalleryMetaDataList {
    #[serde(rename(deserialize = "gmetadata"))]
    items: Vec<GalleryMetaData>
}

#[derive(Deserialize, Debug)]
struct GalleryMetaData {
    gid: u32,
    token: String,
    title: String,
    category: String,
    #[serde(rename(deserialize = "thumb"))]
    thumbnail: String,
    #[serde(rename(deserialize = "filecount"))]
    #[serde(deserialize_with = "from_str")]
    file_count: u32,
    uploader: String,
    #[serde(deserialize_with = "from_tag")]
    tags: Vec<Metatag>
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
    Ok(list_of_tags.iter().map(|e| {
        let (meta, tag) = e.split_once(":").unwrap();
        
        serde_json::from_str::<Metatag>(format!("{{\"{}\": \"{}\"}}", meta.to_string(), tag.to_string()).as_str()).unwrap()
    }).collect::<Vec<_>>())
}