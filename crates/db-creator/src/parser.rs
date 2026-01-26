use std::{fmt::Debug, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Root1 {
    pub error: Option<bool>,
    pub category: Category,
    pub dumped: u64,
    #[serde(default, deserialize_with = "from_optional_string")]
    pub current_gid: Option<u64>,
    pub current_key: Option<String>,
    pub expunged: bool,
    #[serde(deserialize_with = "from_string")]
    pub filecount: u32,
    pub filesize: u64,
    #[serde(default, deserialize_with = "from_optional_string")]
    pub first_gid: Option<u64>,
    pub first_key: Option<String>,
    pub gid: u64,
    #[serde(default, deserialize_with = "from_optional_string")]
    pub parent_gid: Option<u64>,
    pub parent_key: Option<String>,
    #[serde(deserialize_with = "from_string")]
    pub posted: u64,
    #[serde(deserialize_with = "from_string")]
    pub rating: f64,
    pub tags: Vec<Tag>,
    pub thumb: String,
    pub title: String,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub title_jpn: Option<String>,
    pub token: String,
    #[serde(deserialize_with = "from_string")]
    pub torrentcount: u32,
    pub torrents: Vec<Torrents1>,
    pub uploader: String,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Torrents1 {
    #[serde(deserialize_with = "from_string")]
    pub added: u64,
    #[serde(deserialize_with = "from_string")]
    pub fsize: u64,
    pub hash: Box<str>,
    pub name: Option<Box<str>>,
    #[serde(deserialize_with = "from_string")]
    pub tsize: u64,
}

#[derive(Deserialize, Serialize)]
pub enum Category {
    Doujinshi = 0,
    Manga = 1,
    #[serde(rename = "Artist CG")]
    ArtistCG = 2,
    #[serde(rename = "Game CG")]
    GameCG = 3,
    Western = 4,
    #[serde(rename = "Non-H")]
    NonH = 5,
    #[serde(rename = "Image Set")]
    ImageSet = 6,
    Cosplay = 7,
    #[serde(rename = "Asian Porn")]
    AsianPorn = 8,
    Misc = 9,
    #[serde(rename = "private")]
    Private = 10,
}

fn from_optional_string<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        Value::Null => Ok(None),
        Value::String(s) => T::from_str(&s).map(Some).map_err(de::Error::custom),
        Value::Number(number) => T::from_str(&number.to_string())
            .map(Some)
            .map_err(de::Error::custom),
        Value::Bool(_) => todo!(),
        Value::Array(_) => todo!(),
        Value::Object(_) => todo!(),
    }
}
fn from_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => T::from_str(&s).map_err(de::Error::custom),
        Value::Number(number) => T::from_str(&number.to_string()).map_err(de::Error::custom),
        Value::Null => todo!(),
        Value::Bool(_) => todo!(),
        Value::Array(_) => todo!(),
        Value::Object(_) => todo!(),
    }
}

pub fn empty_string_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let opt = Option::<String>::deserialize(deserializer)?;

    match opt {
        None => Ok(None),
        Some(s) if s.trim().is_empty() => Ok(None),
        Some(s) => T::from_str(&s).map(Some).map_err(serde::de::Error::custom),
    }
}

#[derive(Debug)]
pub enum TagPrefix {
    Other,
    Female,
    Male,
    Mixed,
    Language,
    Reclass,
    Parody,
    Character,
    Group,
    Artist,
    Cosplayer,
    Location,
    Temp,
    None,
}
#[derive(Debug)]
pub struct Tag<T: Debug = String> {
    pub tag: T,
    pub prefix: TagPrefix,
}

impl<'de> Deserialize<'de> for Tag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <String>::deserialize(deserializer)?;
        Ok(Tag::from(s.as_str()))
    }
}

impl Serialize for Tag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!(
            "{}:{}",
            match self.prefix {
                TagPrefix::Other => "other",
                TagPrefix::Female => "female",
                TagPrefix::Male => "male",
                TagPrefix::Mixed => "mixed",
                TagPrefix::Language => "language",
                TagPrefix::Reclass => "reclass",
                TagPrefix::Parody => "parody",
                TagPrefix::Character => "character",
                TagPrefix::Group => "group",
                TagPrefix::Artist => "artist",
                TagPrefix::Cosplayer => "cosplayer",
                TagPrefix::Location => "location",
                TagPrefix::Temp => "temp",
                TagPrefix::None => "none",
            },
            self.tag
        ))
    }
}

impl From<&str> for Tag {
    fn from(value: &str) -> Self {
        if !value.contains(":") {
            return Tag {
                tag: value.to_string(),
                prefix: TagPrefix::None,
            };
        }
        let (k, v) = value
            .split_once(":")
            .expect(&format!("Invalid tag format: {}", value));
        let value = v.to_string();
        Tag {
            tag: value,
            prefix: match k {
                "other" => TagPrefix::Other,
                "female" => TagPrefix::Female,
                "male" => TagPrefix::Male,
                "mixed" => TagPrefix::Mixed,
                "language" => TagPrefix::Language,
                "reclass" => TagPrefix::Reclass,
                "parody" => TagPrefix::Parody,
                "character" => TagPrefix::Character,
                "group" => TagPrefix::Group,
                "artist" => TagPrefix::Artist,
                "cosplayer" => TagPrefix::Cosplayer,
                "location" => TagPrefix::Location,
                "temp" => TagPrefix::Temp,
                _ => unimplemented!("{}", k),
            },
        }
    }
}
