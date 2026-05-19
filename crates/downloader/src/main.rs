use std::path::Path;

use chrono::{DateTime, Utc};
use downloader::{
    DetailRequest, Item, Mode, Tag, detail_requests_from_items, filter_missing_detail_requests,
    load_detail_requests, load_missing_detail_requests, parse_detail_args, select_new_data_items,
    write_data_items, write_detail_item, write_detail_requests,
};
use quick_xml::de::from_str;

use reqwest::Client;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let Some(mode) = args.first() else {
        eprintln!("Usage: downloader <data|detail> [--limit N]");
        std::process::exit(2);
    };
    let mode = Mode::parse(mode).map_err(anyhow::Error::msg)?;
    let detail_args = parse_detail_args(&args[1..]).map_err(anyhow::Error::msg)?;

    let client = Client::new();

    match mode {
        Mode::Data => {
            let items = select_new_data_items(fetch_data(&client).await?, Path::new("data"));
            let count = items.len();
            let requests = detail_requests_from_items(&items);
            write_data_items(items, Path::new("data"))?;
            write_detail_requests(&requests, Path::new("detail_requests.json"))?;
            println!("Wrote {count} data item(s)");
        }
        Mode::Detail => {
            let requests = if let Some(path) = detail_args.requests_path.as_deref() {
                let requests = load_detail_requests(path)?;
                filter_missing_detail_requests(requests, Path::new("detail"), detail_args.limit)
            } else {
                load_missing_detail_requests(
                    Path::new("data"),
                    Path::new("detail"),
                    detail_args.limit,
                )?
            };
            let count = requests.len();
            write_detail_items(&client, requests).await?;
            println!("Wrote {count} detail item(s)");
        }
    }

    Ok(())
}

async fn write_detail_items(client: &Client, requests: Vec<DetailRequest>) -> anyhow::Result<()> {
    for chunk in requests.chunks(25) {
        let payload = chunk
            .iter()
            .map(|request| (request.gid, request.token.clone()))
            .collect::<Vec<_>>();
        for (file, gid) in api(client, payload)
            .await?
            .into_iter()
            .zip(chunk.iter().map(|request| request.gid))
        {
            write_detail_item(Path::new("detail"), gid, file, dumped_timestamp())?;
        }
    }
    Ok(())
}

fn dumped_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
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

use scraper::{Html, Selector};
use serde::Deserialize;
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
