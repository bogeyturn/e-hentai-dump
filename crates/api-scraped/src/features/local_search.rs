use std::{fmt::Display, time::Duration, u32};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    Session,
    features::search::{Search, SearchInfo, SearchQuery},
};

#[derive(Serialize)]
pub struct SearchRequest {
    pub filter: String,
    pub page: Option<u64>,
    pub forward: bool,
    pub range: Option<u8>,
    pub date: Option<String>,
    pub jump: Option<String>,
    pub explunged: Option<bool>,
    pub torrent: bool,
    pub min_rating: f64,
    pub min_pages: u32,
    pub max_pages: u32,
    pub category_bits: u16,
}

impl Session {
    pub async fn local_search(&self, sq: SearchQuery) -> anyhow::Result<Search> {
        let f = sq.query.unwrap_or_default();
        let req = if let Some(v) = sq.advanced {
            SearchRequest {
                filter: f,
                page: sq.pid,
                range: sq.range,
                date: sq.seek,
                jump: sq.jump,
                forward: sq.forward,
                explunged: Some(v.explunged.unwrap_or_default()),
                torrent: v.require_torrent.unwrap_or_default(),
                min_rating: (v.min_rating.unwrap_or_default() as f64 / 2.0),
                min_pages: v.min_pages.unwrap_or(0),
                max_pages: v.max_pages.unwrap_or(u32::MAX),
                category_bits: sq.cat.unwrap_or(Category::all().bits()),
            }
        } else {
            SearchRequest {
                filter: f,
                page: sq.pid,
                forward: sq.forward,
                range: sq.range,
                date: sq.seek,
                jump: sq.jump,
                category_bits: sq.cat.unwrap_or(Category::all().bits()),
                explunged: None,
                torrent: false,
                min_rating: 0.0,
                min_pages: 0,
                max_pages: u32::MAX,
            }
        };
        let d: SearchResponse = self.local_api("search", &req).await?.json().await?;

        Ok(Search {
            progress_min: 0.0,
            progress_max: d.progress.unwrap_or_default(),
            items: d
                .items
                .into_iter()
                .map(|v| SearchInfo {
                    img: Some(v.thumb),
                    id: v.gid,
                    token: v.token,
                    category: v.category.to_string(),
                    disowned: v.uploader.is_none(),
                    publisher: v.uploader.unwrap_or_default(),
                    published: Duration::from_secs(v.posted),
                    pages: v.filecount,
                    tags: v.tags.into_iter().map(|v| v.to_string()).collect(),
                    title: v.title,
                    rating: v
                        .your_rating
                        .unwrap_or_else(|| f64::round(v.rating * 2.0) as u8),
                    voted: v.your_rating.is_some(),
                    favorite: v.fav_group.map(|v| v.0),
                    new: false,
                })
                .collect(),
            first: d.first,
            last: d.last,
            count: format!("Found {} results.", d.count.unwrap_or(0)),
        })
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
#[derive(Deserialize)]
pub struct SearchResponse {
    progress: Option<f64>,
    first: bool,
    last: bool,
    count: Option<u64>,
    pub items: Vec<NewItem>,
}

#[derive(Deserialize)]
pub struct Torrents1 {
    pub added: u64,
    pub fsize: u64,
    pub hash: String,
    pub name: Option<String>,
    pub tsize: u64,
}

bitflags::bitflags! {
    #[derive(Clone, Copy)]
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
#[derive(Debug, Serialize)]
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

#[derive(Debug)]
pub struct Tag<T = String> {
    pub tag: T,
    pub prefix: TagPrefix,
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
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
                TagPrefix::None => "",
            },
            self.tag
        )
    }
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

#[derive(Deserialize)]
pub struct NewItem {
    pub gid: u64,
    pub token: String,
    pub first_gid: Option<u64>,
    pub parent_gid: Option<u64>,

    pub title: String,
    pub title_jpn: Option<String>,
    pub thumb: String,
    pub category: Category,
    pub rating: f64,
    pub tags: Vec<Tag>,

    pub filecount: u32,
    pub filesize: u64,
    pub torrentcount: u32,
    pub torrents: Vec<Torrents1>,

    pub uploader: Option<String>,
    pub posted: u64,
    pub dumped: u64,
    pub expunged: bool,

    pub your_rating: Option<u8>,
    pub fav_group: Option<(u8, Option<String>)>,
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
