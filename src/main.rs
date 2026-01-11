use std::{
    fs::{File, create_dir_all},
    io::Write,
    path::PathBuf,
};

use chrono::{DateTime, Utc};
use quick_xml::de::from_str;

use reqwest::Client;
#[tokio::main]
async fn main() {
    let client = Client::new();

    let data = fetch_data(&client)
        .await
        .unwrap()
        .into_iter()
        .map(|v| (PathBuf::from(format!("data/{}.json", v.gid)), v))
        .filter(|v| !v.0.exists())
        .collect::<Vec<_>>();
    create_dir_all("detail").unwrap();
    create_dir_all("data").unwrap();
    for data in data.chunks(25) {
        let payload = data
            .iter()
            .map(|v| (v.1.gid, v.1.token.clone()))
            .collect::<Vec<_>>();
        for (mut file, gid) in api(&client, payload)
            .await
            .unwrap()
            .into_iter()
            .zip(data.iter().map(|v| v.1.gid))
        {
            if let Value::Object(map) = &mut file {
                map.insert(
                    "dumped".to_string(),
                    Value::from(
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    ),
                );
            }
            let path = PathBuf::from(format!("detail/{}.json", gid));
            File::create(path)
                .unwrap()
                .write_all(serde_json::to_string(&file).unwrap().as_bytes())
                .unwrap();
        }
    }
    for (path, item) in data {
        let t = serde_json::to_string(&item).unwrap();
        File::create(path).unwrap().write_all(t.as_bytes()).unwrap();
    }
}

async fn api(client: &Client, ids: Vec<(u64, String)>) -> Result<Vec<Value>, reqwest::Error> {
    let url = "https://api.e-hentai.org/api.php";
    let response = client
        .post(url)
        .json(&json!({
          "method": "gdata",
          "gidlist": ids,
          "namespace": 1
        }))
        .send()
        .await?;
    let data: Data = response.json().await?;
    Ok(data.gmetadata)
}

#[derive(Deserialize)]
struct Data {
    gmetadata: Vec<Value>,
}

#[derive(Debug)]
pub enum Tag {
    Other(String),
    Female(String),
    Male(String),
    Mixed(String),
    Language(String),
    Reclass(String),
    Parody(String),
    Character(String),
    Group(String),
    Artist(String),
    Cosplayer(String),
    Location(String),
    Temp(String),
    None(String),
}

impl Serialize for Tag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            Tag::None(v) => v.clone(),
            Tag::Artist(v) => format!("a:{v}"),
            Tag::Character(v) => format!("c:{v}"),
            Tag::Cosplayer(v) => format!("co:{v}"),
            Tag::Female(v) => format!("f:{v}"),
            Tag::Group(v) => format!("g:{v}"),
            Tag::Language(v) => format!("l:{v}"),
            Tag::Location(v) => format!("lo:{v}"),
            Tag::Male(v) => format!("m:{v}"),
            Tag::Mixed(v) => format!("mi:{v}"),
            Tag::Other(v) => format!("o:{v}"),
            Tag::Parody(v) => format!("p:{v}"),
            Tag::Reclass(v) => format!("r:{v}"),
            Tag::Temp(v) => format!("t:{v}"),
        };

        serializer.serialize_str(&s)
    }
}

impl From<&str> for Tag {
    fn from(value: &str) -> Self {
        if !value.contains(":") {
            return Tag::None(value.to_string());
        }
        let (k, v) = value
            .split_once(":")
            .expect(&format!("Invalid tag format: {}", value));
        let value = v.to_string();
        match k {
            "other" => Tag::Other(value),
            "female" => Tag::Female(value),
            "male" => Tag::Male(value),
            "mixed" => Tag::Mixed(value),
            "language" => Tag::Language(value),
            "reclass" => Tag::Reclass(value),
            "parody" => Tag::Parody(value),
            "character" => Tag::Character(value),
            "group" => Tag::Group(value),
            "artist" => Tag::Artist(value),
            "cosplayer" => Tag::Cosplayer(value),
            "location" => Tag::Location(value),
            "temp" => Tag::Temp(value),
            _ => unimplemented!("{}", k),
        }
    }
}

async fn fetch_data(client: &Client) -> Result<Vec<Item>, anyhow::Error> {
    let response = client
        .get("https://e-hentai.org/rss/ehg.xml")
        .send()
        .await?;
    let body = response.text().await?;
    let html = Html::parse_document(&body);
    let select = Selector::parse("p:nth-child(3)").unwrap();
    let v = html
        .select(&select)
        .map(|v| {
            let v = v.inner_html();
            let v = v
                .strip_prefix("Tags: ")
                .unwrap()
                .split_once("<br><br>Description: ")
                .unwrap();
            (
                v.0.split(", ")
                    .filter(|v| !v.trim().is_empty())
                    .map(|v| Tag::from(v.trim()))
                    .collect::<Vec<_>>(),
                if v.1 == "n/t" {
                    None
                } else {
                    Some(v.1.to_owned())
                },
            )
        })
        .collect::<Vec<_>>();
    let feed: Feed = from_str(&body).unwrap();

    Ok(feed
        .entries
        .into_iter()
        .zip(v)
        .map(|(v, (tags, desc))| {
            let url = v.links[0].href.replace("https://e-hentai.org/g/", "");
            let mut url = url.split("/");
            let img: Div2 = serde_json::from_value(v.content.div).unwrap();
            Item {
                gid: url.next().unwrap().parse().unwrap(),
                token: url.next().unwrap().to_owned(),
                author: v.author.name,
                name: v.title,
                published: {
                    let dt: DateTime<Utc> = v.updated.parse().unwrap();
                    dt.timestamp() as u64
                },
                img: img.img.src,
                description: desc,
                categories: tags,
            }
        })
        .collect())
}

#[derive(Serialize)]
pub struct Item {
    #[serde(rename = "a")]
    author: String,
    #[serde(rename = "c")]
    categories: Vec<Tag>,
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "g")]
    gid: u64,
    #[serde(rename = "i")]
    img: String,
    #[serde(rename = "n")]
    name: String,
    #[serde(rename = "p")]
    published: u64,
    #[serde(rename = "t")]
    token: String,
}

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Debug, Deserialize)]
#[serde(rename = "feed")]
pub struct Feed {
    #[serde(rename = "entry")]
    entries: Vec<Entry>,
}
#[derive(Debug, Deserialize)]
#[serde(rename = "entry", rename_all = "kebab-case")]
pub struct Entry {
    pub title: String,
    #[serde(rename = "link")]
    pub links: Vec<Link>,
    pub id: String,
    pub updated: String,
    pub author: Author,
    pub content: Div,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "content", rename_all = "kebab-case")]
pub struct Div {
    pub div: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct Div2 {
    pub img: Img,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "div", rename_all = "kebab-case")]
pub struct Img {
    #[serde(rename = "@src")]
    pub src: String,
}

#[derive(Debug, Deserialize)]
pub struct Link {
    #[serde(rename = "@rel")]
    pub rel: Option<String>,

    #[serde(rename = "@type")]
    pub link_type: Option<String>,

    #[serde(rename = "@href")]
    pub href: String,
}

#[derive(Debug, Deserialize)]
pub struct Author {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Summary {
    #[serde(rename = "@type")]
    pub summary_type: Option<String>,

    #[serde(rename = "$text")]
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Content {
    #[serde(rename = "$value")]
    pub value: String,
}
