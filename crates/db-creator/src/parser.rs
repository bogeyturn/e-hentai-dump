use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Root1 {
    pub error: Option<bool>,
    #[serde(deserialize_with = "category_string")]
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
    pub hash: String,
    pub name: Option<String>,
    #[serde(deserialize_with = "from_string")]
    pub tsize: u64,
}

use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct Category: u16 {
        const DOUJINSHI   = 0b0000_0000_0001;
        const MANGA       = 0b0000_0000_0010;
        const ARTIST_CG   = 0b0000_0000_0100;
        const GAME_CG     = 0b0000_0000_1000;
        const WESTERN     = 0b0000_0001_0000;
        const NON_H       = 0b0000_0010_0000;
        const IMAGE_SET   = 0b0000_0100_0000;
        const COSPLAY     = 0b0000_1000_0000;
        const ASIAN_PORN  = 0b0001_0000_0000;
        const MISC        = 0b0010_0000_0000;
        const PRIVATE     = 0b0100_0000_0000;
    }
}

const CATEGORY_NAMES: &[(Category, &str)] = &[
    (Category::DOUJINSHI, "Doujinshi"),
    (Category::MANGA, "Manga"),
    (Category::ARTIST_CG, "Artist CG"),
    (Category::GAME_CG, "Game CG"),
    (Category::WESTERN, "Western"),
    (Category::NON_H, "Non-H"),
    (Category::IMAGE_SET, "Image Set"),
    (Category::COSPLAY, "Cosplay"),
    (Category::ASIAN_PORN, "Asian Porn"),
    (Category::MISC, "Misc"),
    (Category::PRIVATE, "private"),
];

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut names = Vec::new();
        for (flag, name) in CATEGORY_NAMES {
            if self.contains(*flag) {
                names.push(*name);
            }
        }
        write!(f, "{}", names.join(", "))
    }
}
impl FromStr for Category {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Doujinshi" => Ok(Category::DOUJINSHI),
            "Manga" => Ok(Category::MANGA),
            "Artist CG" => Ok(Category::ARTIST_CG),
            "Game CG" => Ok(Category::GAME_CG),
            "Western" => Ok(Category::WESTERN),
            "Non-H" => Ok(Category::NON_H),
            "Image Set" => Ok(Category::IMAGE_SET),
            "Cosplay" => Ok(Category::COSPLAY),
            "Asian Porn" => Ok(Category::ASIAN_PORN),
            "Misc" => Ok(Category::MISC),
            "private" => Ok(Category::PRIVATE),
            _ => Err(format!("Invalid category: {}", s)),
        }
    }
}

impl Serialize for Category {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(self.bits())
    }
}

impl<'de> Deserialize<'de> for Category {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let bits = u16::deserialize(deserializer)?;

        Category::from_bits(bits)
            .ok_or_else(|| D::Error::custom(format!("invalid Category bitmask: {bits:#x}")))
    }
}

fn category_string<'de, D>(deserializer: D) -> Result<Category, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    Category::from_str(&value).map_err(de::Error::custom)
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
        Value::Object(_) | Value::Bool(_) | Value::Array(_) => todo!(),
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
        Value::Object(_) | Value::Bool(_) | Value::Array(_) | Value::Null => todo!(),
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
    Other = 0,
    Female = 1,
    Male = 2,
    Mixed = 3,
    Language = 4,
    Reclass = 5,
    Parody = 6,
    Character = 7,
    Group = 8,
    Artist = 9,
    Cosplayer = 10,
    Location = 11,
    Temp = 12,
    None = 13,
}

impl Display for TagPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
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
            }
        )
    }
}

impl From<u8> for TagPrefix {
    fn from(value: u8) -> Self {
        match value {
            0 => TagPrefix::Other,
            1 => TagPrefix::Female,
            2 => TagPrefix::Male,
            3 => TagPrefix::Mixed,
            4 => TagPrefix::Language,
            5 => TagPrefix::Reclass,
            6 => TagPrefix::Parody,
            7 => TagPrefix::Character,
            8 => TagPrefix::Group,
            9 => TagPrefix::Artist,
            10 => TagPrefix::Cosplayer,
            11 => TagPrefix::Location,
            12 => TagPrefix::Temp,
            13 => TagPrefix::None,
            _ => TagPrefix::Other,
        }
    }
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
