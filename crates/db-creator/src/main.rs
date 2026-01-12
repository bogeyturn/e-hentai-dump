use std::{collections::HashSet, fs::File, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize};

fn main() {
    let mut users = HashSet::new();
    let mut tags = HashSet::new();
    for file in std::fs::read_dir("detail").unwrap() {
        let file = file.unwrap().path();
        let file: Root1 = serde_json::from_reader(File::open(file).unwrap()).unwrap();
        users.insert(file.uploader);
        for tag in file.tags {
            tags.insert(tag.tag);
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Torrents1 {
    #[serde(deserialize_with = "from_string")]
    added: u64,
    #[serde(deserialize_with = "from_string")]
    fsize: u64,
    hash: String,
    name: String,
    #[serde(deserialize_with = "from_string")]
    tsize: u64,
}

#[derive(Deserialize, Serialize)]
enum Category {
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
}

fn from_optional_string<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let opt = Option::<String>::deserialize(deserializer)?;

    match opt {
        None => Ok(None),
        Some(s) => T::from_str(&s).map(Some).map_err(serde::de::Error::custom),
    }
}
fn from_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
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
pub struct Tag {
    tag: String,
    prefix: TagPrefix,
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

impl Serialize for Tag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let v = self.tag.clone();
        let s = match self.prefix {
            TagPrefix::None => v.clone(),
            TagPrefix::Artist => format!("a:{v}"),
            TagPrefix::Character => format!("c:{v}"),
            TagPrefix::Cosplayer => format!("co:{v}"),
            TagPrefix::Female => format!("f:{v}"),
            TagPrefix::Group => format!("g:{v}"),
            TagPrefix::Language => format!("l:{v}"),
            TagPrefix::Location => format!("lo:{v}"),
            TagPrefix::Male => format!("m:{v}"),
            TagPrefix::Mixed => format!("mi:{v}"),
            TagPrefix::Other => format!("o:{v}"),
            TagPrefix::Parody => format!("p:{v}"),
            TagPrefix::Reclass => format!("r:{v}"),
            TagPrefix::Temp => format!("t:{v}"),
        };
        serializer.serialize_str(&s)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Root1 {
    category: Category,
    dumped: u64,
    expunged: bool,
    #[serde(deserialize_with = "from_string")]
    filecount: u32,
    filesize: i64,
    #[serde(default, deserialize_with = "from_optional_string")]
    first_gid: Option<u64>,
    first_key: Option<String>,
    gid: i64,
    #[serde(default, deserialize_with = "from_optional_string")]
    parent_gid: Option<u64>,
    parent_key: Option<String>,
    #[serde(deserialize_with = "from_string")]
    posted: u64,
    #[serde(deserialize_with = "from_string")]
    rating: f64,
    tags: Vec<Tag>,
    thumb: String,
    title: String,
    #[serde(deserialize_with = "empty_string_as_none")]
    title_jpn: Option<String>,
    token: String,
    #[serde(deserialize_with = "from_string")]
    torrentcount: u32,
    torrents: Vec<Torrents1>,
    uploader: String,
}
